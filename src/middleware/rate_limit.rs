use crate::{networking, state::AppState};
use axum::{
    extract::Request,
    extract::State,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use moka::future::Cache;
use std::{net::IpAddr, sync::Arc};
use tokio::time::Duration;
use tracing::error;

pub const MAX_REQUESTS_PER_SEC: u32 = 10;

#[derive(Clone)]
pub struct RateLimiter {
    cache: Arc<Cache<IpAddr, u32>>,
    max_requests: u32,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            cache: Arc::new(Cache::builder().time_to_live(window).build()),
            max_requests,
        }
    }
}

/// Rate limit middleware that limits the number of requests per second from a single IP address and
/// uses in-memory caching to store the number of requests.
pub async fn rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let headers = req.headers().clone();
    let client_ip = match networking::get_forwarded_ip(headers.clone()) {
        Some(ip) => ip,
        None => {
            error!(
                "Failed to get forwarded IP from request in rate limiting middleware. Skipping the \
                 rate-limiting."
            );
            // We are skipping the drop to the connect info IP address here, because we are
            // using the Load Balancer and if any issues with the X-Forwarded-IP header, we
            // will rate-limit the LB IP address.
            return next.run(req).await;
        }
    };

    let rate_limiter = &state.rate_limit;
    let mut rate_limit = rate_limiter.cache.get_with(client_ip, async { 0 }).await;

    if rate_limit < rate_limiter.max_requests {
        rate_limit += 1;
        rate_limiter.cache.insert(client_ip, rate_limit).await;
        next.run(req).await
    } else {
        (StatusCode::TOO_MANY_REQUESTS, "Too many requests").into_response()
    }
}

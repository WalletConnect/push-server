use {
    axum::http::{HeaderName, HeaderValue, Request},
    tower_http::request_id::{MakeRequestId, RequestId},
    uuid::Uuid,
};

pub const X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

#[derive(Clone, Default)]
pub struct GenericRequestId;

impl MakeRequestId for GenericRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let uuid = Uuid::new_v4().to_string();
        let request_id =
            HeaderValue::from_str(uuid.as_str()).unwrap_or(HeaderValue::from_static("unknown-f"));
        Some(RequestId::new(request_id))
    }
}

pub fn get_req_id<B>(req: &Request<B>) -> String {
    req
        .headers()
        .get(X_REQUEST_ID)
        // Unknown Missing
        .unwrap_or(&HeaderValue::from_static("unknown-m"))
        .to_str()
        // Unknown Failed
        .unwrap_or("unknown-f")
        .to_string()
}

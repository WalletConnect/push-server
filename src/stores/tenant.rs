use crate::env::Config;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait TenantStore {}

#[async_trait]
impl TenantStore for PgPool {
    // TODO impl
}

pub struct DefaultTenantStore(Arc<Config>);
impl DefaultTenantStore {
    pub fn new(config: Arc<Config>) -> DefaultTenantStore {
        DefaultTenantStore(config)
    }
}

#[async_trait]
impl TenantStore for DefaultTenantStore {
    // TODO impl
}

use std::sync::Arc;
use sqlx::PgPool;
use crate::env::Config;
use async_trait::async_trait;

#[async_trait]
pub trait TenantStore {

}

#[async_trait]
impl TenantStore for PgPool {
    // TODO impl
}

pub struct DefaultTenantStore(Arc<Config>);

#[async_trait]
impl TenantStore for DefaultTenantStore {
    // TODO impl
}
use crate::env::Config;
use crate::error::Error::ProviderNotAvailable;
use crate::error::Result;
use crate::providers::apns::ApnsProvider;
use crate::providers::fcm::FcmProvider;
use crate::providers::noop::NoopProvider;
use crate::providers::{Provider, ProviderKind, Providers};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait TenantStore {
    async fn get_tenant_providers(&self, id: &str) -> Result<Vec<ProviderKind>>;

    async fn get_tenant_provider(&self, id: &str, name: &ProviderKind) -> Result<Provider>;
}

#[async_trait]
impl TenantStore for PgPool {
    async fn get_tenant_providers(&self, _id: &str) -> Result<Vec<ProviderKind>> {
        todo!()
    }

    async fn get_tenant_provider(&self, _id: &str, _name: &ProviderKind) -> Result<Provider> {
        todo!()
    }
}

pub struct DefaultTenantStore {
    config: Arc<Config>,
    fcm: Option<FcmProvider>,
    apns: Option<ApnsProvider>,
    #[cfg(any(debug_assertions, test))]
    noop: Option<NoopProvider>,
}

impl DefaultTenantStore {
    pub fn new(config: Arc<Config>) -> Result<DefaultTenantStore> {
        let providers = Providers::new(&config)?;

        Ok(DefaultTenantStore {
            config,
            fcm: providers.fcm,
            apns: providers.apns,
            #[cfg(any(debug_assertions, test))]
            noop: providers.noop,
        })
    }
}

#[async_trait]
impl TenantStore for DefaultTenantStore {
    async fn get_tenant_providers(&self, _id: &str) -> Result<Vec<ProviderKind>> {
        let mut supported = vec![];

        if self.config.apns_certificate.is_some() && self.config.apns_certificate_password.is_some()
        {
            supported.push(ProviderKind::Apns);
        }

        if self.config.fcm_api_key.is_some() {
            supported.push(ProviderKind::Fcm);
        }

        // Only available in debug/testing
        #[cfg(any(debug_assertions, test))]
        supported.push(ProviderKind::Noop);

        Ok(supported)
    }

    async fn get_tenant_provider(&self, id: &str, name: &ProviderKind) -> Result<Provider> {
        if !self.get_tenant_providers(id).await?.contains(name) {
            return Err(ProviderNotAvailable(name.into()));
        }

        match name {
            ProviderKind::Apns => match self.apns.clone() {
                Some(p) => Ok(Provider::Apns(p)),
                None => Err(ProviderNotAvailable(name.into())),
            },
            ProviderKind::Fcm => match self.fcm.clone() {
                Some(p) => Ok(Provider::Fcm(p)),
                None => Err(ProviderNotAvailable(name.into())),
            },
            #[cfg(any(debug_assertions, test))]
            ProviderKind::Noop => match self.noop.clone() {
                Some(p) => Ok(Provider::Noop(p)),
                None => Err(ProviderNotAvailable(name.into())),
            },
        }
    }
}

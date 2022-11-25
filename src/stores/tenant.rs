use {
    crate::{
        env::Config,
        error::{
            Error::{InvalidTenantId, ProviderNotAvailable},
            Result,
        },
        providers::{
            apns::ApnsProvider,
            fcm::FcmProvider,
            noop::NoopProvider,
            Provider::{self, Apns, Fcm, Noop},
            ProviderKind,
        },
    },
    async_trait::async_trait,
    chrono::{DateTime, Utc},
    sqlx::PgPool,
    std::{io::BufReader, sync::Arc},
};

#[derive(sqlx::FromRow, Debug, Eq, PartialEq, Clone)]
pub struct Tenant {
    id: String,

    fcm_api_key: Option<String>,

    apns_sandbox: bool,
    apns_topic: Option<String>,
    apns_certificate: Option<String>,
    apns_certificate_password: Option<String>,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Tenant {
    pub fn providers(&self) -> Vec<ProviderKind> {
        let mut supported = vec![];

        if self.apns_certificate.is_some() && self.apns_certificate_password.is_some() {
            supported.push(ProviderKind::Apns);
        }

        if self.fcm_api_key.is_some() {
            supported.push(ProviderKind::Fcm);
        }

        // Only available in debug/testing
        #[cfg(any(debug_assertions, test))]
        supported.push(ProviderKind::Noop);

        supported
    }

    pub fn provider(&self, provider: &ProviderKind) -> Result<Provider> {
        if !self.providers().contains(provider) {
            return Err(ProviderNotAvailable(provider.into()));
        }

        match provider {
            ProviderKind::Apns => {
                let endpoint = match self.apns_sandbox {
                    true => a2::Endpoint::Sandbox,
                    false => a2::Endpoint::Production,
                };
                match (
                    &self.apns_certificate,
                    &self.apns_certificate_password,
                    &self.apns_topic,
                ) {
                    (Some(certificate), Some(password), Some(topic)) => {
                        let decoded = base64::decode(certificate)?;
                        let mut reader = BufReader::new(&*decoded);

                        let apns_client = ApnsProvider::new_cert(
                            &mut reader,
                            password.clone(),
                            endpoint,
                            topic.clone(),
                        )?;

                        Ok(Apns(apns_client))
                    }
                    _ => Err(ProviderNotAvailable(provider.into())),
                }
            }
            ProviderKind::Fcm => match self.fcm_api_key.clone() {
                Some(api_key) => {
                    let fcm = FcmProvider::new(api_key);
                    Ok(Fcm(fcm))
                }
                None => Err(ProviderNotAvailable(provider.into())),
            },
            #[cfg(any(debug_assertions, test))]
            ProviderKind::Noop => Ok(Noop(NoopProvider::new())),
        }
    }
}

#[async_trait]
pub trait TenantStore {
    async fn get_tenant(&self, id: &str) -> Result<Tenant>;
}

#[async_trait]
impl TenantStore for PgPool {
    async fn get_tenant(&self, id: &str) -> Result<Tenant> {
        let res = sqlx::query_as::<sqlx::postgres::Postgres, Tenant>(
            "SELECT * FROM public.tenants WHERE id = $1",
        )
        .bind(id)
        .fetch_one(self)
        .await;

        match res {
            Err(sqlx::Error::RowNotFound) => Err(InvalidTenantId(id.into())),
            Err(e) => Err(e.into()),
            Ok(row) => Ok(row),
        }
    }
}

pub struct DefaultTenantStore(Tenant);

impl DefaultTenantStore {
    pub fn new(config: Arc<Config>) -> Result<DefaultTenantStore> {
        Ok(DefaultTenantStore(Tenant {
            id: config.default_tenant_id.clone(),
            fcm_api_key: config.fcm_api_key.clone(),
            apns_sandbox: config.apns_sandbox,
            apns_topic: config.apns_topic.clone(),
            apns_certificate: config.apns_certificate.clone(),
            apns_certificate_password: config.apns_certificate_password.clone(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }))
    }
}

#[async_trait]
impl TenantStore for DefaultTenantStore {
    async fn get_tenant(&self, _id: &str) -> Result<Tenant> {
        Ok(self.0.clone())
    }
}

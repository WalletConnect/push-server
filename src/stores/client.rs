use {
    crate::{
        providers::ProviderKind,
        stores::{self, StoreError::NotFound},
    },
    async_trait::async_trait,
    sqlx::Executor,
};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Client {
    pub tenant_id: String,
    pub push_type: ProviderKind,
    #[sqlx(rename = "device_token")]
    pub token: String,
}

#[async_trait]
pub trait ClientStore {
    async fn create_client(&self, tenant_id: &str, id: &str, client: Client) -> stores::Result<()>;
    async fn get_client(&self, tenant_id: &str, id: &str) -> stores::Result<Client>;
    async fn delete_client(&self, tenant_id: &str, id: &str) -> stores::Result<()>;
}

#[async_trait]
impl ClientStore for sqlx::PgPool {
    async fn create_client(&self, tenant_id: &str, id: &str, client: Client) -> stores::Result<()> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO public.clients (id, tenant_id, push_type, device_token)",
        );
        query_builder.push_values(
            vec![(id, tenant_id, client.push_type, client.token)],
            |mut b, client| {
                b.push_bind(client.0)
                    .push_bind(client.1)
                    .push_bind(client.2)
                    .push_bind(client.3);
            },
        );
        query_builder.push(
            " ON CONFLICT (id) DO UPDATE SET device_token = EXCLUDED.device_token, tenant_id = \
             EXCLUDED.tenant_id, push_type = EXCLUDED.push_type",
        );
        let query = query_builder.build();

        self.execute(query).await?;

        Ok(())
    }

    async fn get_client(&self, tenant_id: &str, id: &str) -> stores::Result<Client> {
        let res = sqlx::query_as::<sqlx::postgres::Postgres, Client>(
            "SELECT tenant_id, push_type, device_token FROM public.clients WHERE id = $1 and \
             tenant_id = $2",
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_one(self)
        .await;

        match res {
            Err(sqlx::Error::RowNotFound) => Err(NotFound("client".to_string(), id.to_string())),
            Err(e) => Err(e.into()),
            Ok(row) => Ok(row),
        }
    }

    async fn delete_client(&self, tenant_id: &str, id: &str) -> stores::Result<()> {
        let mut notification_query_builder =
            sqlx::QueryBuilder::new("DELETE FROM public.notifications WHERE client_id = ");
        notification_query_builder.push_bind(id);
        notification_query_builder.push(" and tenant_id = ");
        notification_query_builder.push_bind(tenant_id);
        let notification_query = notification_query_builder.build();

        self.execute(notification_query).await?;

        let mut query_builder = sqlx::QueryBuilder::new("DELETE FROM public.clients WHERE id = ");
        query_builder.push_bind(id);
        query_builder.push(" and tenant_id = ");
        query_builder.push_bind(tenant_id);
        let query = query_builder.build();

        match self.execute(query).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

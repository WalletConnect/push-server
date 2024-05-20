use {
    crate::{
        metrics::Metrics,
        providers::ProviderKind,
        stores::{self, StoreError::NotFound},
    },
    async_trait::async_trait,
    sqlx::Executor,
    std::time::Instant,
    tracing::{debug, instrument},
};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Client {
    pub tenant_id: String,
    pub push_type: ProviderKind,
    #[sqlx(rename = "device_token")]
    pub token: String,
    pub always_raw: bool,
}

#[async_trait]
pub trait ClientStore {
    async fn create_client(
        &self,
        tenant_id: &str,
        id: &str,
        client: Client,
        metrics: Option<&Metrics>,
    ) -> stores::Result<()>;
    async fn get_client(&self, tenant_id: &str, id: &str) -> stores::Result<Client>;
    async fn delete_client(&self, tenant_id: &str, id: &str) -> stores::Result<()>;
}

#[async_trait]
impl ClientStore for sqlx::PgPool {
    #[instrument(skip(self, client, metrics))]
    async fn create_client(
        &self,
        tenant_id: &str,
        id: &str,
        client: Client,
        metrics: Option<&Metrics>,
    ) -> stores::Result<()> {
        debug!(
            "ClientStore::create_client tenant_id={tenant_id} id={id} token={} with locking",
            client.token
        );

        let mut transaction = self.begin().await?;

        // Statement for locking based on the client id to prevent an issue #230
        // and locking based on the token to prevent an issue #292
        let start = Instant::now();
        sqlx::query(
            "SELECT
                pg_advisory_xact_lock(abs(hashtext($1::text))),
                pg_advisory_xact_lock(abs(hashtext($2::text)))",
        )
        .bind(id)
        .bind(client.token.clone())
        .execute(&mut transaction)
        .await?;
        if let Some(metrics) = metrics {
            metrics.postgres_query("create_client_pg_advisory_xact_lock", start);
        }

        let start = Instant::now();
        sqlx::query("DELETE FROM public.clients WHERE id = $1 OR device_token = $2")
            .bind(id)
            .bind(client.token.clone())
            .execute(&mut transaction)
            .await?;
        if let Some(metrics) = metrics {
            metrics.postgres_query("create_client_delete", start);
        }

        let start = Instant::now();
        let mut insert_query = sqlx::QueryBuilder::new(
            "INSERT INTO public.clients (id, tenant_id, push_type, device_token, always_raw)",
        );
        insert_query.push_values(
            vec![(
                id,
                tenant_id,
                client.push_type,
                client.token,
                client.always_raw,
            )],
            |mut b, client| {
                b.push_bind(client.0)
                    .push_bind(client.1)
                    .push_bind(client.2)
                    .push_bind(client.3)
                    .push_bind(client.4);
            },
        );
        insert_query.build().execute(&mut transaction).await?;
        if let Some(metrics) = metrics {
            metrics.postgres_query("create_client_insert", start);
        }

        let start = Instant::now();
        transaction.commit().await?;
        if let Some(metrics) = metrics {
            metrics.postgres_query("create_client_commit", start);
        }

        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_client(&self, tenant_id: &str, id: &str) -> stores::Result<Client> {
        let res = sqlx::query_as::<sqlx::postgres::Postgres, Client>(
            "SELECT tenant_id, push_type, device_token, always_raw FROM public.clients WHERE id = \
             $1 and tenant_id = $2",
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

    #[instrument(skip(self))]
    async fn delete_client(&self, tenant_id: &str, id: &str) -> stores::Result<()> {
        debug!("ClientStore::delete_client tenant_id={tenant_id} id={id}");

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

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

        #[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
        pub struct ClientSelect {
            pub id: String,
            pub device_token: String,
            pub tenant_id: String,
        }

        let start = Instant::now();
        let mut transaction = self.begin().await?;
        if let Some(metrics) = metrics {
            metrics.postgres_query("create_client_begin", start);
        }

        let query = "
            SELECT *
            FROM public.clients
            WHERE id = $1
                  OR device_token = $2
            FOR UPDATE
        ";
        let start = Instant::now();
        let existing_client = sqlx::query_as::<sqlx::postgres::Postgres, ClientSelect>(query)
            .bind(id)
            .bind(client.token.clone())
            .fetch_one(&mut transaction)
            .await
            .map(Some)
            .or_else(|e| match e {
                sqlx::Error::RowNotFound => Ok(None),
                e => Err(e),
            })?;
        if let Some(metrics) = metrics {
            metrics.postgres_query("create_client_select", start);
        }

        #[cfg(feature = "functional_tests")]
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        if let Some(existing_client) = existing_client {
            if existing_client.id == id && existing_client.device_token != client.token {
                let query = "
                    UPDATE public.clients
                    SET device_token = $2,
                        push_type = $3,
                        always_raw = $4,
                        tenant_id = $5
                    WHERE id = $1
                ";
                let start = Instant::now();
                sqlx::query(query)
                    .bind(id)
                    .bind(client.token)
                    .bind(client.push_type)
                    .bind(client.always_raw)
                    .bind(tenant_id)
                    .execute(&mut transaction)
                    .await?;
                if let Some(metrics) = metrics {
                    metrics.postgres_query("create_client_update_device_token", start);
                }
            } else if existing_client.device_token == client.token && existing_client.id != id {
                let query = "
                    DELETE FROM public.notifications
                    WHERE client_id = $1
                          AND tenant_id = $2
                ";
                let start = Instant::now();
                sqlx::query(query)
                    .bind(existing_client.id)
                    .bind(existing_client.tenant_id)
                    .execute(&mut transaction)
                    .await?;
                if let Some(metrics) = metrics {
                    metrics.postgres_query("create_client_delete_notifications", start);
                }

                let query = "
                    UPDATE public.clients
                    SET id = $2,
                        push_type = $3,
                        always_raw = $4,
                        tenant_id = $5
                    WHERE device_token = $1
                ";
                let start = Instant::now();
                sqlx::query(query)
                    .bind(client.token)
                    .bind(id)
                    .bind(client.push_type)
                    .bind(client.always_raw)
                    .bind(tenant_id)
                    .execute(&mut transaction)
                    .await?;
                if let Some(metrics) = metrics {
                    metrics.postgres_query("create_client_update_id", start);
                }
            } else {
                let query = "
                    UPDATE public.clients
                    SET push_type = $2,
                        always_raw = $3,
                        tenant_id = $4
                    WHERE id = $1
                ";
                let start = Instant::now();
                sqlx::query(query)
                    .bind(id)
                    .bind(client.push_type)
                    .bind(client.always_raw)
                    .bind(tenant_id)
                    .execute(&mut transaction)
                    .await?;
                if let Some(metrics) = metrics {
                    metrics.postgres_query("create_client_update_id", start);
                }
            }
        } else {
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

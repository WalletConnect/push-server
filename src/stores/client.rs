use crate::providers::ProviderKind;
use crate::stores;
use crate::stores::StoreError::NotFound;
use async_trait::async_trait;
use sqlx::Executor;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Client {
    pub push_type: ProviderKind,
    #[sqlx(rename = "device_token")]
    pub token: String,
}

#[async_trait]
pub trait ClientStore {
    async fn create_client(&self, id: &str, client: Client) -> stores::Result<()>;
    async fn get_client(&self, id: &str) -> stores::Result<Client>;
    async fn delete_client(&self, id: &str) -> stores::Result<()>;
}

#[async_trait]
impl ClientStore for sqlx::PgPool {
    async fn create_client(&self, id: &str, client: Client) -> stores::Result<()> {
        let mut query_builder =
            sqlx::QueryBuilder::new("INSERT INTO public.clients (id, push_type, device_token) ");
        query_builder.push_values(
            vec![(id, client.push_type, client.token)],
            |mut b, client| {
                b.push_bind(client.0)
                    .push_bind(client.1)
                    .push_bind(client.2);
            },
        );
        let query = query_builder.build();

        self.execute(query).await?;

        Ok(())
    }

    async fn get_client(&self, id: &str) -> stores::Result<Client> {
        let res = sqlx::query_as::<sqlx::postgres::Postgres, Client>(
            "SELECT push_type, device_token FROM public.clients WHERE id = $1",
        )
        .bind(id)
        .fetch_one(self)
        .await;

        match res {
            Err(sqlx::Error::RowNotFound) => Err(NotFound("client".to_string(), id.to_string())),
            Err(e) => Err(e.into()),
            Ok(row) => Ok(row),
        }
    }

    async fn delete_client(&self, id: &str) -> stores::Result<()> {
        let mut notification_query_builder =
            sqlx::QueryBuilder::new("DELETE FROM public.notifications WHERE client_id = ");
        notification_query_builder.push_bind(id);
        let notification_query = notification_query_builder.build();

        self.execute(notification_query).await?;

        let mut query_builder = sqlx::QueryBuilder::new("DELETE FROM public.clients WHERE id = ");
        query_builder.push_bind(id);
        let query = query_builder.build();

        match self.execute(query).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

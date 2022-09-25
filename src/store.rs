use crate::error;
use async_trait::async_trait;
use sqlx::Executor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    pub push_type: String,
    pub token: String,
}

#[async_trait]
pub trait ClientStore {
    async fn create_client(&self, id: &str, client: Client) -> error::Result<()>;
    async fn get_client(&self, id: &str) -> error::Result<Option<Client>>;
    async fn delete_client(&self, id: &str) -> error::Result<()>;
}

#[async_trait]
impl ClientStore for sqlx::PgPool {
    async fn create_client(&self, id: &str, client: Client) -> error::Result<()> {
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

    async fn get_client(&self, _id: &str) -> error::Result<Option<Client>> {
        // TODO Get Clients
        Ok(None)
    }

    async fn delete_client(&self, id: &str) -> error::Result<()> {
        let mut query_builder = sqlx::QueryBuilder::new("DELETE FROM public.clients WHERE id = ");
        query_builder.push_bind(id);
        let query = query_builder.build();

        self.execute(query).await?;

        Ok(())
    }
}

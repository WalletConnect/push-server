use crate::error;
use async_trait::async_trait;
use sqlx::Executor;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    pub push_type: String,
    pub token: String,
}

#[async_trait]
pub trait ClientStore {
    async fn init(&mut self) -> error::Result<()>;
    async fn create_client(&mut self, id: &str, client: Client) -> error::Result<()>;
    async fn get_client(&self, id: &str) -> error::Result<Option<&Client>>;
    async fn delete_client(&mut self, id: &str) -> error::Result<()>;
}

#[async_trait]
impl<K> ClientStore for HashMap<K, Client>
where
    K: Into<String> + From<String> + Eq + Hash + Send + Sync,
{
    async fn init(&mut self) -> error::Result<()> {
        Ok(())
    }

    async fn create_client(&mut self, id: &str, client: Client) -> error::Result<()> {
        self.insert(K::from(id.to_string()), client);
        Ok(())
    }

    async fn get_client(&self, id: &str) -> error::Result<Option<&Client>> {
        let client = self.get(&K::from(id.to_string()));
        Ok(client)
    }

    async fn delete_client(&mut self, id: &str) -> error::Result<()> {
        self.remove(&K::from(id.to_string()));
        Ok(())
    }
}

const INIT_SQL: &str = include_str!("../sql/00-init.sql");
const CREATE_CLIENTS_SQL: &str = include_str!("../sql/01-create-clients.sql");
const CREATE_NOTIFICATIONS_SQL: &str = include_str!("../sql/02-create-notifications.sql");

#[async_trait]
impl ClientStore for sqlx::PgPool {
    async fn init(&mut self) -> error::Result<()> {
        self.execute(sqlx::QueryBuilder::new(INIT_SQL).build())
            .await?;
        self.execute(sqlx::QueryBuilder::new(CREATE_CLIENTS_SQL).build())
            .await?;
        self.execute(sqlx::QueryBuilder::new(CREATE_NOTIFICATIONS_SQL).build())
            .await?;
        Ok(())
    }

    async fn create_client(&mut self, id: &str, client: Client) -> error::Result<()> {
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

    async fn get_client(&self, id: &str) -> error::Result<Option<&Client>> {
        // TODO Get Clients
        Ok(None)
    }

    async fn delete_client(&mut self, id: &str) -> error::Result<()> {
        let mut query_builder = sqlx::QueryBuilder::new("DELETE FROM public.clients WHERE id = ");
        query_builder.push_bind(id);
        let query = query_builder.build();

        self.execute(query).await?;

        Ok(())
    }
}

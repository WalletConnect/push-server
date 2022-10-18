use crate::error;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Executor;
use sqlx::types::Json;
use crate::handlers::push_message::MessagePayload;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Notification {
    id: String,
    client_id: String,

    last_payload: Json<MessagePayload>,
    previous_payloads: Vec<Json<MessagePayload>>,

    last_received_at: DateTime<Utc>,
    created_at: DateTime<Utc>
}

#[async_trait]
pub trait NotificationStore {
    async fn create_or_update_notification(
        &self,
        id: &str,
        client: Notification,
    ) -> error::Result<()>;
    async fn get_notification(&self, id: &str) -> error::Result<Option<Notification>>;
    async fn delete_notification(&self, id: &str) -> error::Result<()>;
}

#[async_trait]
impl NotificationStore for sqlx::PgPool {
    async fn create_or_update_notification(
        &self,
        _id: &str,
        _client: Notification,
    ) -> error::Result<()> {
        todo!()
    }

    async fn get_notification(&self, id: &str) -> error::Result<Option<Notification>> {
        let res = sqlx::query_as::<sqlx::postgres::Postgres, Notification>(
            "SELECT * FROM public.notifications WHERE id = $1",
        )
            .bind(id)
            .fetch_one(self)
            .await;

        match res {
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e.into()),
            Ok(row) => Ok(Some(row)),
        }
    }

    async fn delete_notification(&self, id: &str) -> error::Result<()> {
        let mut query_builder = sqlx::QueryBuilder::new("DELETE FROM public.notifications WHERE id = ");
        query_builder.push_bind(id);
        let query = query_builder.build();

        self.execute(query).await?;

        Ok(())
    }
}

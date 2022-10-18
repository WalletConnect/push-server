use crate::error;
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Notification {}

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

    async fn get_notification(&self, _id: &str) -> error::Result<Option<Notification>> {
        todo!()
    }

    async fn delete_notification(&self, _id: &str) -> error::Result<()> {
        todo!()
    }
}

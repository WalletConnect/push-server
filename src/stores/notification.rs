use {
    crate::{
        handlers::push_message::MessagePayload,
        stores::{self, StoreError::NotFound},
    },
    async_trait::async_trait,
    chrono::{DateTime, Utc},
    sqlx::{types::Json, Executor},
};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Notification {
    pub id: String,
    pub client_id: String,

    pub last_payload: Json<MessagePayload>,
    pub previous_payloads: Vec<Json<MessagePayload>>,

    pub last_received_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait NotificationStore {
    async fn create_or_update_notification(
        &self,
        id: &str,
        tenant_id: &str,
        client_id: &str,
        payload: &MessagePayload,
    ) -> stores::Result<Notification>;
    async fn get_notification(
        &self,
        id: &str,
        client_id: &str,
        tenant_id: &str,
    ) -> stores::Result<Notification>;
    async fn delete_notification(&self, id: &str, tenant_id: &str) -> stores::Result<()>;
}

#[async_trait]
impl NotificationStore for sqlx::PgPool {
    async fn create_or_update_notification(
        &self,
        id: &str,
        tenant_id: &str,
        client_id: &str,
        payload: &MessagePayload,
    ) -> stores::Result<Notification> {
        let res = sqlx::query_as::<sqlx::postgres::Postgres, Notification>(
            "
            INSERT INTO public.notifications (id, tenant_id, client_id, last_payload)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id, client_id)
                DO UPDATE SET last_received_at  = now()
            RETURNING *;",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(client_id)
        .bind(Json(payload))
        .fetch_one(self)
        .await;

        match res {
            Err(e) => Err(e.into()),
            Ok(row) => Ok(row),
        }
    }

    async fn get_notification(
        &self,
        id: &str,
        client_id: &str,
        tenant_id: &str,
    ) -> stores::Result<Notification> {
        let res = sqlx::query_as::<sqlx::postgres::Postgres, Notification>(
            "
            SELECT *
            FROM public.notifications
            WHERE id = $1 AND client_id = $2 AND tenant_id = $3",
        )
        .bind(id)
        .bind(client_id)
        .bind(tenant_id)
        .fetch_one(self)
        .await;

        match res {
            Err(sqlx::Error::RowNotFound) => {
                Err(NotFound("notification".to_string(), id.to_string()))
            }
            Err(e) => Err(e.into()),
            Ok(row) => Ok(row),
        }
    }

    async fn delete_notification(&self, id: &str, tenant_id: &str) -> stores::Result<()> {
        let mut query_builder =
            sqlx::QueryBuilder::new("DELETE FROM public.notifications WHERE id = ");
        query_builder.push_bind(id);
        query_builder.push("and tenant_id = ");
        query_builder.push_bind(tenant_id);
        let query = query_builder.build();

        self.execute(query).await?;

        Ok(())
    }
}

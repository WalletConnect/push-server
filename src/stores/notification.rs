use {
    crate::{
        handlers::push_message::PushMessageBody,
        stores::{self, StoreError::NotFound},
    },
    async_trait::async_trait,
    chrono::{DateTime, Utc},
    serde_json::Value,
    sqlx::{types::Json, Executor},
    tracing::instrument,
};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Notification {
    pub id: String,
    pub client_id: String,

    pub last_payload: Json<Value>,
    pub previous_payloads: Vec<Json<Value>>,

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
        payload: &PushMessageBody,
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
    #[instrument(skip(self, payload))]
    async fn create_or_update_notification(
        &self,
        id: &str,
        tenant_id: &str,
        client_id: &str,
        payload: &PushMessageBody,
    ) -> stores::Result<Notification> {
        let mut transaction = self.begin().await?;

        sqlx::query("SELECT pg_advisory_xact_lock(abs(hashtext($1::text)))")
            .bind(client_id)
            .execute(&mut transaction)
            .await?;

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
        .fetch_one(&mut transaction)
        .await;

        transaction.commit().await?;

        match res {
            Err(e) => Err(e.into()),
            Ok(row) => Ok(row),
        }
    }

    #[instrument(skip(self))]
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

    #[instrument(skip(self))]
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

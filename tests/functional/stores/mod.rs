use uuid::Uuid;

mod client;
mod notification;
/// Tests against the stores
mod tenant;

pub const TENANT_ID: &str = "000-000-000-000";

pub fn gen_id() -> String {
    Uuid::new_v4().to_string()
}

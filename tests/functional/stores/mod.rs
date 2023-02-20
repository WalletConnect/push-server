use uuid::Uuid;

mod client;
mod notification;
/// Tests against the stores
mod tenant;

pub fn gen_id() -> String {
    Uuid::new_v4().to_string()
}

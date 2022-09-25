use async_trait::async_trait;
use echo_server::{
    error,
    providers::PROVIDER_NOOP,
    store::{Client, ClientStore},
};
use std::{collections::HashMap, sync::Mutex};

const ID: &str = "0000-0000-0000-0000";
const PUSH_TOKEN: &str = "0000-0000-0000-0000";

#[derive(Default)]
pub struct MockStore(Mutex<HashMap<String, Client>>);

impl MockStore {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl ClientStore for MockStore {
    async fn create_client(&self, id: &str, client: Client) -> error::Result<()> {
        self.0.lock().unwrap().insert(id.into(), client);
        Ok(())
    }

    async fn get_client(&self, id: &str) -> error::Result<Option<Client>> {
        let lock = self.0.lock().unwrap();
        let client = lock.get(id).map(Clone::clone);
        Ok(client)
    }

    async fn delete_client(&self, id: &str) -> error::Result<()> {
        self.0.lock().unwrap().remove(id);
        Ok(())
    }
}

/// Create a test store using the `HashMap` impl of `ClientStore`
fn setup_test_store() -> Box<impl ClientStore> {
    Box::new(MockStore::default())
}

/// Insert a default client into the `setup_test_store` store
async fn setup_filled_test_store() -> Box<impl ClientStore> {
    let store = setup_test_store();

    store
        .create_client(
            ID,
            Client {
                push_type: PROVIDER_NOOP.to_owned(),
                token: PUSH_TOKEN.to_owned(),
            },
        )
        .await
        .expect("Failed to insert test client during setup phase");

    store
}

#[tokio::test]
async fn insert_client() {
    let store = setup_test_store();

    let res = store
        .create_client(
            ID,
            Client {
                push_type: PROVIDER_NOOP.to_owned(),
                token: PUSH_TOKEN.to_owned(),
            },
        )
        .await
        .expect("Failed to insert client into store");

    assert_eq!(res, ())
}

#[tokio::test]
async fn fetch_client() {
    let store = setup_filled_test_store().await;

    let client = store
        .get_client(&ID.to_string())
        .await
        .expect("Failed to fetch client");

    assert_eq!(client.is_some(), true);
    assert_eq!(
        client.unwrap(),
        Client {
            push_type: PROVIDER_NOOP.to_owned(),
            token: PUSH_TOKEN.to_owned(),
        }
    )
}

#[tokio::test]
async fn delete_client() {
    let store = setup_filled_test_store().await;

    let res = store
        .delete_client(&ID.to_owned())
        .await
        .expect("Failed to delete client from store");

    assert_eq!(res, ())
}

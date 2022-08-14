#[cfg(test)]
mod store_test {
    use crate::store::ClientStore;
    use crate::Client;
    use std::collections::HashMap;

    const ID: &str = "0000-0000-0000-0000";
    const PUSH_TOKEN: &str = "0000-0000-0000-0000";

    /// Create a test store using the `HashMap` impl of `ClientStore`
    fn setup_test_store() -> Box<impl ClientStore> {
        let data: HashMap<String, Client> = HashMap::new();
        Box::new(data)
    }

    /// Insert a default client into the `setup_test_store` store
    fn setup_filled_test_store() -> Box<impl ClientStore> {
        let mut store = setup_test_store();

        store
            .create_client(
                ID.to_string(),
                Client {
                    push_type: "noop".to_string(),
                    token: PUSH_TOKEN.to_string(),
                },
            )
            .expect("Failed to insert test client during setup phase");

        store
    }

    #[test]
    fn insert_client() {
        let mut store = setup_test_store();

        let res = store
            .create_client(
                ID.to_string(),
                Client {
                    push_type: "noop".to_string(),
                    token: PUSH_TOKEN.to_string(),
                },
            )
            .expect("Failed to insert client into store");

        assert_eq!(res, ())
    }

    #[test]
    fn fetch_client() {
        let store = setup_filled_test_store();

        let client = store
            .get_client(&ID.to_string())
            .expect("Failed to fetch client");

        assert_eq!(client.is_some(), true);
        assert_eq!(
            client.unwrap(),
            &Client {
                push_type: "noop".to_string(),
                token: PUSH_TOKEN.to_string(),
            }
        )
    }

    #[test]
    fn delete_client() {
        let mut store = setup_filled_test_store();

        let res = store
            .delete_client(&ID.to_string())
            .expect("Failed to delete client from store");

        assert_eq!(res, ())
    }
}

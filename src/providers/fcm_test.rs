#[cfg(test)]
mod fcm_test {
    use crate::providers::fcm;
    use crate::providers::{get_provider, Provider};

    #[test]
    fn fetch_provider() {
        let provider = get_provider("fcm".to_string());
        if provider.is_err() {
            panic!("Failed to fetch fcm provider")
        }

        match provider.unwrap() {
            Provider::Fcm(p) => {
                assert_eq!(p, fcm::new())
            }
            _ => panic!("`get_provider` didn't return a fcm provider"),
        }
    }
}

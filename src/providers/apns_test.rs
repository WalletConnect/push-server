#[cfg(test)]
mod apns_test {
    use crate::providers::apns;
    use crate::providers::{get_provider, Provider};

    #[test]
    fn fetch_provider() {
        let provider = get_provider("apns".to_string()).expect("Failed to fetch apns provider");

        match provider {
            Provider::Apns(p) => {
                assert_eq!(p, apns::new())
            }
            _ => panic!("`get_provider` didn't return an apns provider"),
        }
    }
}

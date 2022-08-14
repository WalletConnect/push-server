use crate::providers::PushProvider;

pub struct ApnsProvider {}

pub fn new() -> ApnsProvider {
    ApnsProvider {}
}

impl PushProvider for ApnsProvider {
    fn send_notification(&mut self, _token: String, _message: String) -> crate::error::Result<()> {
        todo!()
    }
}

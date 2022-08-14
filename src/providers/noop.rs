use crate::providers::PushProvider;

pub struct NoopProvider {}

pub fn new() -> NoopProvider {
    NoopProvider {}
}

impl PushProvider for NoopProvider {
    fn send_notification(&mut self, _message: String) -> crate::error::Result<()> {
        todo!()
    }
}

use crate::providers::PushProvider;

#[derive(Debug, PartialEq)]
pub struct FcmProvider {}

pub fn new() -> FcmProvider {
    FcmProvider {}
}

impl PushProvider for FcmProvider {
    fn send_notification(&mut self, _token: String, _message: String) -> crate::error::Result<()> {
        todo!()
    }
}

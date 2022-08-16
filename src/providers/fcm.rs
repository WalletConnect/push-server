use crate::providers::PushProvider;

#[derive(Debug, PartialEq, Clone)]
pub struct FcmProvider {}

impl FcmProvider {
    pub fn new() -> Self {
        FcmProvider {}
    }
}

impl PushProvider for FcmProvider {
    fn send_notification(&mut self, _token: String, _message: String) -> crate::error::Result<()> {
        todo!()
    }
}

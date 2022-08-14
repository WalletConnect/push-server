use crate::providers::PushProvider;

pub struct FcmProvider {}

pub fn new() -> FcmProvider {
    FcmProvider {}
}

impl PushProvider for FcmProvider {
    fn send_notification(_message: String) -> crate::error::Result<()> {
        todo!()
    }
}

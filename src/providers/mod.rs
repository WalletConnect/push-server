use crate::error::Error::ProviderNotFound;

pub trait PushProvider {
    fn send_notification(message: String) -> crate::error::Result<()>;
}

pub fn get_provider<P>(name: String) -> crate::error::Result<P>
where
    P: PushProvider,
{
    match name {
        _ => Err(ProviderNotFound(name)),
    }
}

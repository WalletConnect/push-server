use crate::providers::PushProvider;
use a2::NotificationBuilder;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct ApnsProvider {
    client: a2::Client,
}

impl ApnsProvider {
    pub fn new_cert<R>(
        cert: &mut R,
        password: String,
        endpoint: a2::Endpoint,
    ) -> crate::error::Result<Self>
    where
        R: Read,
    {
        Ok(ApnsProvider {
            client: a2::Client::certificate(cert, password.as_str(), endpoint)?,
        })
    }

    pub fn new_token<R>(
        pem: &mut R,
        key_id: String,
        team_id: String,
        endpoint: a2::Endpoint,
    ) -> crate::error::Result<Self>
    where
        R: Read,
    {
        Ok(ApnsProvider {
            client: a2::Client::token(pem, key_id, team_id, endpoint)?,
        })
    }
}

impl PushProvider for ApnsProvider {
    fn send_notification(&mut self, token: String, message: String) -> crate::error::Result<()> {
        let opt = a2::NotificationOptions::default();

        let notification =
            a2::PlainNotificationBuilder::new(message.as_str()).build(token.as_str(), opt);

        let _res = self.client.send(notification);

        Ok(())
    }
}

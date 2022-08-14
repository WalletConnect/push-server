use crate::error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Client {
    pub push_type: String,
    pub token: String,
}

pub trait ClientStore<K> {
    fn create_client(&mut self, id: K, client: Client) -> error::Result<()>;
    fn get_client(&self, id: &K) -> error::Result<Option<&Client>>;
    fn delete_client(&mut self, id: &K) -> error::Result<()>;
}

impl<K> ClientStore<K> for HashMap<K, Client> {
    fn create_client(&mut self, id: K, client: Client) -> error::Result<()> {
        self.insert(id, client);
        Ok(())
    }

    fn get_client(&self, id: &K) -> error::Result<Option<&Client>> {
        let client = self.get(id);
        Ok(client)
    }

    fn delete_client(&mut self, id: &K) -> error::Result<()> {
        self.remove(id);
        Ok(())
    }
}

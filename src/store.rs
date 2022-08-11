use crate::error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Client {
    pub push_type: String,
    pub token: String,
}

pub trait ClientStore {
    fn create_client(&mut self, id: String, client: Client) -> error::Result<()>;
    fn get_client(&self, id: String) -> error::Result<Option<&Client>>;
    fn delete_client(&mut self, id: String) -> error::Result<()>;
}

impl ClientStore for HashMap<String, Client> {
    fn create_client(&mut self, id: String, client: Client) -> error::Result<()> {
        self.insert(id, client);
        Ok(())
    }

    fn get_client(&self, id: String) -> error::Result<Option<&Client>> {
        let client = self.get(&*id);
        Ok(client)
    }

    fn delete_client(&mut self, id: String) -> error::Result<()> {
        self.remove(&*id);
        Ok(())
    }
}

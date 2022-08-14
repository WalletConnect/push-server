use crate::error;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Client {
    pub push_type: String,
    pub token: String,
}

pub trait ClientStore {
    fn create_client(&mut self, id: String, client: Client) -> error::Result<()>;
    fn get_client(&self, id: &String) -> error::Result<Option<&Client>>;
    fn delete_client(&mut self, id: &String) -> error::Result<()>;
}

impl<K> ClientStore for HashMap<K, Client>
where
    K: Into<String> + From<String> + Eq + Hash,
{
    fn create_client(&mut self, id: String, client: Client) -> error::Result<()> {
        self.insert(K::from(id), client);
        Ok(())
    }

    fn get_client(&self, id: &String) -> error::Result<Option<&Client>> {
        let client = self.get(&K::from(id.clone()));
        Ok(client)
    }

    fn delete_client(&mut self, id: &String) -> error::Result<()> {
        self.remove(&K::from(id.clone()));
        Ok(())
    }
}

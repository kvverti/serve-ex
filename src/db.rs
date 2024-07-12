use std::{sync::{RwLock, Arc}, collections::HashMap};

use uuid::Uuid;

use crate::data::Receipt;


/// A "database connection". In a real application, this would connect to an actual database and wouldn't have terrible scaling properties.
#[derive(Debug, Clone)]
pub struct Connection {
    /// The receipts in our database.
    receipts: Arc<RwLock<HashMap<Uuid, Receipt>>>,
}


/// Implementation of our fake connection. In a real application, this would make remote calls to the database, hence the
/// functions being marked `async`.
impl Connection {
    /// Constructs a new "connection" to the "database".
    pub fn new() -> Self {
        Self {
            receipts: Default::default(),
        }
    }

    /// Stores the data for a receipt in the database, returning its database ID.
    /// If the receipt cannot be stored, returns None.
    pub async fn store_receipt(&self, receipt: Receipt) -> Option<Uuid> {
        if !receipt.is_acceptable() {
            return None;
        }
        let id = Uuid::new_v4();
        self.receipts.write().unwrap().insert(id, receipt);
        Some(id)
    }

    /// Loads a receipt by ID from the database. Returns None if there is no receipt for the ID.
    pub async fn load_receipt(&self, id: Uuid) -> Option<Receipt> {
        // cloning the underlying receipt here because in a real database we'd be constructing a new value.
        self.receipts.read().unwrap().get(&id).cloned()
    }
}
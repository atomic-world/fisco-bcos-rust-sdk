use std::sync::{Arc, RwLock};

use uuid::Uuid;

pub struct EventLogParam {
    filter_id: Arc<String>,
    from_block: Arc<RwLock<String>>,
    to_block: Arc<RwLock<String>>,
    addresses: Arc<RwLock<Vec<String>>>,
    topics: Arc<RwLock<Vec<String>>>,
}

impl EventLogParam {
    pub fn new() -> EventLogParam {
        EventLogParam {
            filter_id: Arc::new(Uuid::new_v4().to_string().replace("-", "")),
            from_block: Arc::new(RwLock::new(String::from("latest"))),
            to_block: Arc::new(RwLock::new(String::from("latest"))),
            addresses: Arc::new(RwLock::new(vec![])),
            topics: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn get_filter_id(&self) -> String {
        self.filter_id.clone().to_string()
    }

    pub fn get_from_block(&self) -> String {
        let from_block_lock = self.from_block.clone();
        let from_block_read_lock = from_block_lock.read().unwrap();
        from_block_read_lock.to_string()
    }

    pub fn set_from_block(&self, from_block: &str) {
        let from_block_lock = self.from_block.clone();
        let mut from_block_write_lock = from_block_lock.write().unwrap();
        *from_block_write_lock = from_block.to_owned();
    }

    pub fn get_to_block(&self) -> String {
        let to_block_lock = self.to_block.clone();
        let to_block_read_lock = to_block_lock.read().unwrap();
        to_block_read_lock.to_string()
    }

    pub fn set_to_block(&self, to_block: &str) {
        let to_block_lock = self.from_block.clone();
        let mut to_block_write_lock = to_block_lock.write().unwrap();
        *to_block_write_lock = to_block.to_owned();
    }

    pub fn get_addresses(&self) -> Vec<String> {
        let addresses_lock = self.addresses.clone();
        let addresses_read_lock = addresses_lock.read().unwrap();
        addresses_read_lock.to_vec()
    }

    pub fn add_address(&self, address: &str) {
        let addresses_lock = self.addresses.clone();
        let mut addresses_write_lock = addresses_lock.write().unwrap();
        addresses_write_lock.push(address.to_string());
    }

    pub fn remove_address(&self, address: &str) {
        let addresses_lock = self.addresses.clone();
        let mut addresses_write_lock = addresses_lock.write().unwrap();

        let mut removed_indexes: Vec<usize> = vec![];
        for (index, value) in addresses_write_lock.iter().enumerate() {
            if value.eq(address) {
                removed_indexes.push(index);
            }
        }
        for removed_index in removed_indexes {
            addresses_write_lock.remove(removed_index);
        }
    }

    pub fn get_topics(&self) -> Vec<String> {
        let topics_lock = self.topics.clone();
        let topics_read_lock = topics_lock.read().unwrap();
        topics_read_lock.to_vec()
    }

    pub fn add_topic(&self, topic: &str) {
        let topics_lock = self.topics.clone();
        let mut topics_write_lock = topics_lock.write().unwrap();
        topics_write_lock.push(topic.to_owned());
    }

    pub fn remove_topic(&self, topic: &str) {
        let topics_lock = self.topics.clone();
        let mut topics_write_lock = topics_lock.write().unwrap();

        let mut removed_indexes: Vec<usize> = vec![];
        for (index, value) in topics_write_lock.iter().enumerate() {
            if value.eq(topic) {
                removed_indexes.push(index);
            }
        }
        for removed_index in removed_indexes {
            topics_write_lock.remove(removed_index);
        }
    }
}

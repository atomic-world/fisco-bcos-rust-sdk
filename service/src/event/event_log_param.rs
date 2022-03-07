use uuid::Uuid;
use std::cell::RefCell;

pub struct EventLogParam {
    pub(crate) filter_id: String,
    pub(crate) from_block: RefCell<String>,
    pub(crate) to_block: RefCell<String>,
    pub(crate) addresses: RefCell<Vec<String>>,
    pub(crate) topics: RefCell<Vec<String>>,
}

impl EventLogParam {
    pub fn new() -> EventLogParam {
        EventLogParam {
            filter_id: Uuid::new_v4().to_string().replace("-", ""),
            from_block: RefCell::new(String::from("latest")),
            to_block: RefCell::new(String::from("latest")),
            addresses: RefCell::new(vec![]),
            topics: RefCell::new(vec![]),
        }
    }

    pub fn set_from_block(&self, from_block: &str) {
        *self.from_block.borrow_mut() = from_block.to_owned();
    }

    pub fn set_to_block(&self, to_block: &str) {
        *self.to_block.borrow_mut() = to_block.to_owned();
    }

    pub fn add_address(&self, address: &str) {
        self.addresses.borrow_mut().push(address.to_string());
    }

    pub fn remove_address(&self, address: &str) {
        let mut removed_indexes: Vec<usize> = vec![];
        for (index, value) in self.addresses.borrow().iter().enumerate() {
            if value.eq(address) {
                removed_indexes.push(index);
            }
        }
        let mut addresses = self.addresses.borrow_mut();
        for removed_index in removed_indexes {
            addresses.remove(removed_index);
        }
    }

    pub fn add_topic(&self, topic: &str) {
        self.topics.borrow_mut().push(topic.to_owned());
    }

    pub fn remove_topic(&self, topic: &str) {
        let mut removed_indexes: Vec<usize> = vec![];
        for (index, value) in self.topics.borrow().iter().enumerate() {
            if value.eq(topic) {
                removed_indexes.push(index);
            }
        }
        let mut topics = self.topics.borrow_mut();
        for removed_index in removed_indexes {
            topics.remove(removed_index);
        }
    }
}
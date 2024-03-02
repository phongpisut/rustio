use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;

#[derive(serde::Serialize, Clone, Debug)]
pub struct Message {
    pub data: String,
    pub user: String,
}

impl Default for Toggle {
    fn default() -> Self {
        Toggle { items: String::default() }
    }
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct Toggle {
    pub items: String,
}

pub type RoomStore = HashMap<String, VecDeque<Message>>;
pub type ItemStore = HashMap<String, Toggle>;

#[derive(Default)]
pub struct MessageStore {
    pub messages: RwLock<RoomStore>,
    pub items : RwLock<ItemStore>
}

impl MessageStore {
    pub async fn insert(&self, room: &str, message: Message) {
        let mut binding = self.messages.write().await;
        let messages = binding.entry(room.to_owned()).or_default();
        
        for msg in messages.iter_mut() {
            if msg.user == message.user {
                *msg = message; 
                return;
            }
        }
        messages.push_front(message);
    }

    pub async fn remove(&self, room: &str, user: String) {
        let mut binding = self.messages.write().await;
        let messages = binding.entry(room.to_owned()).or_default();
        messages.retain(|x| x.user != user);
    }


    pub async fn get(&self, room: &str) -> Vec<Message> {
        let messages = self.messages.read().await.get(room).cloned();
        messages.unwrap_or_default().into_iter().rev().collect()
    }

    pub async fn get_items(&self, room: &str) -> Toggle {
        let items = self.items.read().await.get(room).cloned();
        items.unwrap_or_default()
        
    } 

    pub async fn set_items(&self, room: &str, items: Toggle) {
        let mut binding = self.items.write().await;
        binding.insert(room.to_owned(), items);
    }

    
}
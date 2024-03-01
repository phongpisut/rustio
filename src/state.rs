use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;

#[derive(serde::Serialize, Clone, Debug)]
pub struct Message {
    pub data: String,
    pub user: String,
}

pub type RoomStore = HashMap<String, VecDeque<Message>>;

#[derive(Default)]
pub struct MessageStore {
    pub messages: RwLock<RoomStore>,
}

impl MessageStore {
    pub async fn insert(&self, room: &str, message: Message) {
        let mut binding = self.messages.write().await;
        let messages = binding.entry(room.to_owned()).or_default();
        
        
        //update existing message by message.user
        messages.retain(|x| x.user != message.user);
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

    
}
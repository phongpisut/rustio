use std::collections::HashMap;
use std::collections::VecDeque;
use serde::Deserialize;
use tokio::sync::RwLock;

#[derive(serde::Serialize, Clone, Debug)]
pub struct Message {
    pub data: String,
    pub user: String,
}


#[derive(serde::Serialize, Deserialize,Clone, Debug)]
pub struct Items {
    pub status: bool,
    pub update_by: String,
}



pub type RoomStore = HashMap<String, VecDeque<Message>>;
pub type ItemStore =  HashMap<String, Items>;

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

    pub async fn get_items(&self) -> ItemStore {
        let items = self.items.read().await;
        items.clone()
    } 

    pub async fn set_items(&self, items: ItemStore) {
        let mut _items = self.items.write().await;
        _items.extend(items);
        
    }

    
}
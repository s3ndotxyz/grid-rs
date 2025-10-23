use grid_rs::{kvs::Storage, ws::WebSocketServer};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    user_id: String,
    content: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Default)]
struct ChatState {
    messages: Vec<Message>,
    user_count: u32,
}

#[grid_rs::main]
fn main(_input: &[u8]) -> Result<Vec<u8>, String> {
    let ws = WebSocketServer::create()
        .map_err(|e| format!("Failed to create server: {e:?}"))?;
    
    let mut state: ChatState = Storage::get("chat", "state")
        .and_then(|data| serde_json::from_slice(&data).ok())
        .unwrap_or_default();
    
    let mut connections = Vec::new();
    let mut message_queue = VecDeque::new();
    
    loop {
        if let Ok(Some(conn)) = ws.accept() {
            state.user_count += 1;
            connections.push(conn);
            
            if let Ok(state_bytes) = serde_json::to_vec(&state) {
                Storage::put("chat", "state", &state_bytes);
            }
        }
        
        connections.retain(|conn| {
            match conn.receive() {
                Ok(Some(msg)) => {
                    message_queue.push_back(msg);
                    true
                }
                // handle dropped connections
                Ok(None) => true,
                Err(_) => {
                    state.user_count = state.user_count.saturating_sub(1);
                    false
                }
            }
        });
        
        while let Some(msg) = message_queue.pop_front() {
            if let Ok(message) = serde_json::from_slice::<Message>(&msg) {
                state.messages.push(message.clone());
                
                if state.messages.len() > 100 {
                    state.messages.remove(0);
                }
                
                if let Ok(state_bytes) = serde_json::to_vec(&state) {
                    Storage::put("chat", "state", &state_bytes);
                }
                
                if let Ok(msg_bytes) = serde_json::to_vec(&message) {
                    connections.retain(|conn| {
                        conn.send(conn.id, &msg_bytes).is_ok()
                    });
                }
            }
        }
    }
}

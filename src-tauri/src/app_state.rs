use std::sync::mpsc::Sender;

use tauri::utils::config::AppConfig;

use crate::message::Message;

pub struct AppState {
    config: AppConfig,
    sender: Option<Sender<Message>>,
}

impl AppState {
    pub fn sender(&self) -> Option<Sender<Message>> {
        self.sender.clone()
    }
}

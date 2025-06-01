use std::{collections::HashMap, path::PathBuf, sync::mpsc, thread};

use anyhow::Error;
use oneshot::Sender;
use taskchampion::{storage::AccessMode::ReadWrite, Replica, StorageConfig, Task, Uuid};

use crate::{
    config::AppConfig,
    utils::{create_task, delete_task, update_task},
};

pub enum Message {
    CreateTask(HashMap<String, String>, Sender<MessageResult>),
    GetAllTasks(Sender<MessageResult>),
    GetTaskById(Uuid, Sender<MessageResult>),
    UpdateTaskById(Uuid, HashMap<String, String>, Sender<MessageResult>),
    DeleteTaskById(Uuid, Sender<MessageResult>),
}

#[derive(Debug)]
pub enum MessageResult {
    One(Task),
    Many(HashMap<Uuid, Task>),
    Err(String),
    None,
}

impl Message {
    pub fn execute(self, replica: &mut Replica) -> Result<(), oneshot::SendError<MessageResult>> {
        match self {
            Message::CreateTask(map, sender) => match create_task(replica, map) {
                Ok(task) => sender.send(MessageResult::One(task)),
                Err(e) => sender.send(MessageResult::Err(e.to_string())),
            },
            Message::GetAllTasks(sender) => match replica.all_tasks() {
                Ok(tasks) => sender.send(MessageResult::Many(tasks)),
                Err(e) => sender.send(MessageResult::Err(e.to_string())),
            },
            Message::GetTaskById(uuid, sender) => match replica.get_task(uuid) {
                Ok(task) => {
                    let result = if let Some(task) = task {
                        MessageResult::One(task)
                    } else {
                        MessageResult::None
                    };
                    sender.send(result)
                }
                Err(e) => sender.send(MessageResult::Err(e.to_string())),
            },
            Message::UpdateTaskById(uuid, map, sender) => match update_task(replica, uuid, map) {
                Ok(task) => sender.send(MessageResult::One(task)),
                Err(e) => sender.send(MessageResult::Err(e.to_string())),
            },
            Message::DeleteTaskById(uuid, sender) => match delete_task(replica, uuid) {
                Ok(task) => sender.send(MessageResult::One(task)),
                Err(e) => sender.send(MessageResult::Err(e.to_string())),
            },
        }
    }
}

pub fn spawn_receiver(config: AppConfig) -> Result<mpsc::Sender<Message>, Error> {
    let (tx, rx) = mpsc::channel::<Message>();
    thread::spawn(move || {
        let storage_config = StorageConfig::OnDisk {
            taskdb_dir: PathBuf::from(config.store()),
            create_if_missing: true,
            access_mode: ReadWrite,
        }
        .into_storage()
        .expect("failed to create storage config");
        let mut replica = Replica::new(storage_config);
        match rx.recv() {
            Ok(msg) => {
                if let Err(e) = msg.execute(&mut replica) {
                    log::error!("Failed to execute the message : {:?}", e);
                }
            }
            Err(e) => log::error!("Failed to receive the message : {:?}", e),
        }
    });
    Ok(tx)
}

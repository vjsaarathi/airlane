use std::collections::HashMap;

use taskchampion::Uuid;
use tauri::State;

use crate::app_state::AppState;
use crate::message::{Message, MessageResult};

type TaskResult = HashMap<String, String>;

#[tauri::command]
pub fn create_task_cmd(
    state: State<'_, AppState>,
    map: HashMap<String, String>,
) -> Result<TaskResult, String> {
    let state = state.inner();
    let sender = state.sender();
    let (tx, rx) = oneshot::channel();
    if sender.is_none() {
        Err(String::from("App is not initialized yet"))
    } else {
        let sender = sender.unwrap();
        if sender.send(Message::CreateTask(map, tx)).is_err() {
            Err(String::from("Error sending the command to backend"))
        } else {
            match rx.recv() {
                Ok(task) => match task {
                    MessageResult::One(task) => {
                        let mut map = HashMap::new();
                        let data = task.into_task_data();
                        for (k, v) in data.iter() {
                            map.insert(k.clone(), v.clone());
                        }
                        Ok(map)
                    }
                    MessageResult::Err(error) => {
                        Err(format!("Error occured while creating the task {:?}", error))
                    }
                    _ => Err(String::from("Unexpected result from the backend")),
                },
                Err(_) => todo!(),
            }
        }
    }
}

#[tauri::command]
pub fn get_all_task_cmd(
    state: State<'_, AppState>,
) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let state = state.inner();
    let sender = state.sender();
    let (tx, rx) = oneshot::channel();
    if sender.is_none() {
        Err(String::from("App is not initialized yet"))
    } else {
        let sender = sender.unwrap();
        if sender.send(Message::GetAllTasks(tx)).is_err() {
            Err(String::from("Error sending the command to backend"))
        } else {
            match rx.recv() {
                Ok(task) => match task {
                    MessageResult::Many(tasks) => {
                        let mut maps = HashMap::new();
                        for (id, task) in tasks {
                            let data = task.into_task_data();
                            let mut map = HashMap::new();
                            for (k, v) in data.iter() {
                                map.insert(k.clone(), v.clone());
                            }
                            maps.insert(id.to_string(), map);
                        }
                        Ok(maps)
                    }
                    MessageResult::Err(error) => {
                        Err(format!("Error occured while creating the task {:?}", error))
                    }
                    _ => Err(String::from("Unexpected result from the backend")),
                },
                Err(_) => todo!(),
            }
        }
    }
}

#[tauri::command]
pub fn get_task_by_id_cmd(state: State<'_, AppState>, uuid: Uuid) -> Result<TaskResult, String> {
    let state = state.inner();
    let sender = state.sender();
    let (tx, rx) = oneshot::channel();
    if sender.is_none() {
        Err(String::from("App is not initialized yet"))
    } else {
        let sender = sender.unwrap();
        if sender.send(Message::GetTaskById(uuid, tx)).is_err() {
            Err(String::from("Error sending the command to backend"))
        } else {
            match rx.recv() {
                Ok(task) => match task {
                    MessageResult::One(task) => {
                        let mut map = HashMap::new();
                        let data = task.into_task_data();
                        for (k, v) in data.iter() {
                            map.insert(k.clone(), v.clone());
                        }
                        Ok(map)
                    }
                    MessageResult::Err(error) => {
                        Err(format!("Error occured while creating the task {:?}", error))
                    }
                    _ => Err(String::from("Unexpected result from the backend")),
                },
                Err(_) => todo!(),
            }
        }
    }
}

#[tauri::command]
pub fn update_task_by_id_cmd(
    state: State<'_, AppState>,
    uuid: Uuid,
    map: HashMap<String, String>,
) -> Result<TaskResult, String> {
    let state = state.inner();
    let sender = state.sender();
    let (tx, rx) = oneshot::channel();
    if sender.is_none() {
        Err(String::from("App is not initialized yet"))
    } else {
        let sender = sender.unwrap();
        if sender.send(Message::UpdateTaskById(uuid, map, tx)).is_err() {
            Err(String::from("Error sending the command to backend"))
        } else {
            match rx.recv() {
                Ok(task) => match task {
                    MessageResult::One(task) => {
                        let mut map = HashMap::new();
                        let data = task.into_task_data();
                        for (k, v) in data.iter() {
                            map.insert(k.clone(), v.clone());
                        }
                        Ok(map)
                    }
                    MessageResult::Err(error) => {
                        Err(format!("Error occured while creating the task {:?}", error))
                    }
                    _ => Err(String::from("Unexpected result from the backend")),
                },
                Err(_) => todo!(),
            }
        }
    }
}

#[tauri::command]
pub fn delete_task_by_id_cmd(state: State<'_, AppState>, uuid: Uuid) -> Result<TaskResult, String> {
    let state = state.inner();
    let sender = state.sender();
    let (tx, rx) = oneshot::channel();
    if sender.is_none() {
        Err(String::from("App is not initialized yet"))
    } else {
        let sender = sender.unwrap();
        if sender.send(Message::DeleteTaskById(uuid, tx)).is_err() {
            Err(String::from("Error sending the command to backend"))
        } else {
            match rx.recv() {
                Ok(task) => match task {
                    MessageResult::One(task) => {
                        let mut map = HashMap::new();
                        let data = task.into_task_data();
                        for (k, v) in data.iter() {
                            map.insert(k.clone(), v.clone());
                        }
                        Ok(map)
                    }
                    MessageResult::Err(error) => {
                        Err(format!("Error occured while creating the task {:?}", error))
                    }
                    _ => Err(String::from("Unexpected result from the backend")),
                },
                Err(_) => todo!(),
            }
        }
    }
}

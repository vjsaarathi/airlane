use std::collections::HashMap;

use anyhow::{anyhow, Error, Result};
use taskchampion::{Operation, Replica, Task, Uuid};

pub fn create_task(replica: &mut Replica, map: HashMap<String, String>) -> Result<Task, Error> {
    let uuid = Uuid::new_v4();
    let mut ops = vec![];
    let mut task = replica.create_task(uuid, &mut ops)?;
    for (field, value) in map {
        let value = if value.trim().is_empty() {
            None
        } else {
            Some(value)
        };
        task.set_value(field, value, &mut ops)?;
    }
    replica.commit_operations(ops)?;
    Ok(task)
}

pub fn update_task(
    replica: &mut Replica,
    uuid: Uuid,
    map: HashMap<String, String>,
) -> Result<Task, Error> {
    let task = replica.get_task(uuid)?;
    let mut ops = vec![Operation::UndoPoint];
    if task.is_none() {
        Err(anyhow!("no task found with uuid {}", uuid))
    } else {
        let mut task = task.unwrap();
        for (field, value) in map {
            let value = if value.trim().is_empty() {
                None
            } else {
                Some(value)
            };
            task.set_value(field, value, &mut ops)?;
        }
        Ok(task)
    }
}

pub fn delete_task(replica: &mut Replica, uuid: Uuid) -> Result<Task, Error> {
    let task = replica.get_task(uuid)?;
    let mut ops = vec![Operation::UndoPoint];
    if task.is_none() {
        Err(anyhow!("no task found with uuid {}", uuid))
    } else {
        let mut task = task.unwrap();
        task.set_status(taskchampion::Status::Deleted, &mut ops)?;
        Ok(task)
    }
}

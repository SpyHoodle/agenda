use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};
use colored::*;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    Inbox,
    Pending,
    Active,
    Complete,
}

impl Status {
    pub fn as_string(&self) -> ColoredString {
        match self {
            Status::Inbox => "📮 Inbox".blue(),
            Status::Pending => "📅 Pending".yellow(),
            Status::Active => "✍️ Active".red(),
            Status::Complete => "📗 Complete".green(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,               // The required title of the task
    pub status: Status,              // Current status of the task
    pub notes: Option<String>,           // Any notes to explain the task
    pub tags: Option<Vec<String>>,          // Tasks can be tagged for organisation
    pub subtasks: Option<Vec<Task>>,     // Tasks can be hierarchically split into subtasks
    pub when: Option<NaiveDateTime>,     // The date you want to do the task
    pub deadline: Option<NaiveDateTime>, // The latest date the task should be done
    pub reminder: Option<NaiveDateTime>, // The datetime a reminder will alert you
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub path: String,             // Where the tasks are stored
    pub tasks: Option<Vec<Task>>, // All the tasks in one vector
}

impl Task {
    pub fn new(title: String, notes: Option<String>, tags: Option<Vec<String>>, when: Option<NaiveDateTime>, deadline: Option<NaiveDateTime>, reminder: Option<NaiveDateTime>) -> Self {
        let status = if when.is_some() {
            Status::Pending
        } else {
            Status::Inbox
        };

        Self {
            title,
            status,
            notes,
            tags,
            subtasks: None,
            when,
            deadline,
            reminder,
        }
    }
}

impl Tasks {
    pub fn new(tasks_path: &str) -> Self {
        Self {
            path: String::from(tasks_path),
            tasks: None
        }
    }

    pub fn get_task(&mut self, id: usize) -> Result<&mut Task, &str> {
        if self.tasks.is_none() {
            Err("there are no tasks")
        } else {
            if id >= self.tasks.as_ref().unwrap().len() {
                Err("couldn't find task")
            } else {
                let task = &mut self.tasks.as_mut().unwrap()[id];
                Ok(task)
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_status(&mut self, id: usize, status: Status) {
        let mut task: &mut Task = self.get_task(id).unwrap();
        task.status = status;
    }

    pub fn add(&mut self, task: Task) {
        if self.tasks.is_none() {
            self.tasks = Some(vec![task]);
        } else {
            self.tasks.as_mut().unwrap().push(task);
        };
    }

    pub fn del(&mut self, id: usize) {
        self.tasks.as_mut().unwrap().remove(id);
    }

    pub fn len(&self) -> usize {
        self.tasks.as_ref().unwrap().len()
    }

    pub fn clear(&mut self) {
        self.tasks = None;
    }
}


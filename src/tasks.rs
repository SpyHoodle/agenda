use chrono::NaiveDateTime;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TasksError(String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Inbox,
    Pending,
    Active,
    Complete,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,                   // The required title of the task
    pub status: Status,                  // Current status of the task
    pub notes: Option<String>,           // Any notes to explain the task
    pub tags: Option<Vec<String>>,       // Tasks can be tagged for organisation
    pub when: Option<NaiveDateTime>,     // The date you want to do the task
    pub deadline: Option<NaiveDateTime>, // The latest date the task should be done
    pub reminder: Option<NaiveDateTime>, // The datetime a reminder will alert you
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub path: String,             // Path to the tasks repository
    pub file: String,             // Path to the tasks file in the repository
    pub tasks: Option<Vec<Task>>, // All the tasks in one vector
}

fn task_not_found(id: usize) -> TasksError {
    TasksError(format!("couldn't find task with id {}", id))
}

fn no_tasks_available() -> TasksError {
    TasksError(String::from("no tasks available"))
}

impl Task {
    pub fn new(
        title: String,
        notes: Option<String>,
        tags: Option<Vec<String>>,
        when: Option<NaiveDateTime>,
        deadline: Option<NaiveDateTime>,
        reminder: Option<NaiveDateTime>,
    ) -> Self {
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
            when,
            deadline,
            reminder,
        }
    }

    pub fn modify(
        &mut self,
        title: Option<String>,
        notes: Option<String>,
        tags: Option<Vec<String>>,
        when: Option<NaiveDateTime>,
        deadline: Option<NaiveDateTime>,
        reminder: Option<NaiveDateTime>,
    ) {
        if let Some(title) = title {
            self.title = title;
        };

        if let Some(notes) = notes {
            self.notes = Some(notes);
        };

        if let Some(tags) = tags {
            self.tags = Some(tags);
        };

        if let Some(when) = when {
            self.when = Some(when);
        };

        if let Some(deadline) = deadline {
            self.deadline = Some(deadline);
        };

        if let Some(reminder) = reminder {
            self.reminder = Some(reminder);
        };
    }

    pub fn start(&mut self) {
        self.status = Status::Active;
    }

    pub fn stop(&mut self) {
        if self.when.is_some() {
            self.status = Status::Inbox;
        } else {
            self.status = Status::Pending;
        }
    }

    pub fn complete(&mut self) {
        self.status = Status::Complete;
    }
}

impl Tasks {
    pub fn new(repo_path: &str, tasks_file: &str) -> Self {
        Self {
            path: String::from(repo_path),
            file: String::from(tasks_file),
            tasks: None,
        }
    }

    pub fn task_exists(&self, id: usize) -> bool {
        id < self.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_task(&mut self, id: usize) -> Result<&mut Task, TasksError> {
        if self.is_empty() {
            Err(no_tasks_available())
        } else if self.task_exists(id) {
            Ok(&mut self.tasks.as_mut().unwrap()[id])
        } else {
            Err(task_not_found(id))
        }
    }

    pub fn push(&mut self, task: Task) {
        if self.is_empty() {
            self.tasks = Some(vec![task]);
        } else {
            self.tasks.as_mut().unwrap().push(task);
        };
    }

    pub fn remove(&mut self, id: usize) -> Result<(), TasksError> {
        if self.task_exists(id) {
            self.tasks.as_mut().unwrap().remove(id);
            Ok(())
        } else {
            Err(task_not_found(id))
        }
    }

    pub fn len(&self) -> usize {
        if self.tasks.is_none() {
            0
        } else {
            self.tasks.as_ref().unwrap().len()
        }
    }

    pub fn clear(&mut self) -> Result<(), TasksError> {
        if self.is_empty() {
            Err(no_tasks_available())
        } else {
            self.tasks = None;
            Ok(())
        }
    }
}

impl Status {
    pub fn as_string(&self) -> ColoredString {
        match self {
            Status::Inbox => "📮 Inbox".blue(),
            Status::Pending => "📅 Pending".yellow(),
            Status::Active => "🕑 Active".red(),
            Status::Complete => "📗 Complete".green(),
        }
    }
}

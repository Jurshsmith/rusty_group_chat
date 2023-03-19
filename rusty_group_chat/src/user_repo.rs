use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Mutex};

use crate::User;

#[derive(Deserialize, Serialize)]
pub enum UserRepoError {
    UserAlreadyExists,
}

impl From<UserRepoError> for String {
    fn from(value: UserRepoError) -> Self {
        match value {
            UserRepoError::UserAlreadyExists => String::from("Already Exists"),
        }
    }
}

pub struct UserRepo {
    // Mutex for thread safety
    users: Mutex<HashSet<String>>,
}

impl UserRepo {
    pub fn new() -> Self {
        UserRepo {
            users: Mutex::new(HashSet::new()),
        }
    }

    pub fn add_user(&self, user: &User) -> Result<(), UserRepoError> {
        let mut users = self.users.lock().unwrap();

        if users.contains(&user.name) {
            Err(UserRepoError::UserAlreadyExists)
        } else {
            users.insert(user.name.clone());

            Ok(())
        }
    }

    pub fn remove_user(&self, user: User) {
        self.users.lock().unwrap().remove(&user.name);
    }
}

impl Default for UserRepo {
    fn default() -> Self {
        Self::new()
    }
}

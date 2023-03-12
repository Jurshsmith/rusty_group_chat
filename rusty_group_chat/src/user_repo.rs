use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Mutex};

use crate::User;

#[derive(Deserialize, Serialize)]
pub enum UserRepoError {
    UserAlreadyExists,
}

impl Into<String> for UserRepoError {
    fn into(self) -> String {
        match self {
            UserRepoError::UserAlreadyExists => "Already exists".to_string(),
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

    pub fn remove_user(&self, user: User) -> Result<(), ()> {
        self.users.lock().unwrap().remove(&user.name);

        Ok(())
    }
}

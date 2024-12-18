use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a User in the system.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl User {
    /// Creates a new User instance with a unique UUID.
    pub fn new(name: String, email: String) -> Self {
        User {
            id: Uuid::new_v4(),
            name,
            email,
        }
    }
}

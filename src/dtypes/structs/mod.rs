pub mod error_message;
pub mod article;

use serde::{Deserialize, Serialize};

pub use self::error_message::ErrorMessage;
pub use self::article::Article;

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    pub id: Option<i32>,
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(id) = self.id {
            return write!(f, "Id: {}", id);
        }
        write!(f, "Id: None")
    }
}
use serde::{Serialize, Deserialize};

use crate::word::Word;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id : i32,
    pub words : Vec<Word>
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub ru_name : String,
    pub eng_name : String
}
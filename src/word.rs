
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Word {
    pub ru_name : String,
    pub eng_name : String,
    pub w : f32
}

fn sigmoid(val : f32) -> f32 {
    1.0 / (1.0 + (-val).exp())
}

impl Word {

    pub fn P(&self) -> f32 {
        return sigmoid(self.w);
    }

    pub fn pickup_rate(&self) -> f32 {
        1.0 - self.P()
    }
}
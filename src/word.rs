
use serde::{Deserialize, Serialize};
use rand::Rng;

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

    pub fn sample(words : &Vec<Word>) -> Word {
        let mut sum = 0.0;
        for w in words {
            sum += w.pickup_rate();
        }

        let mut rnd = rand::thread_rng();

        let sample = rnd.gen_range(0.0..sum);

        let mut local_sum = 0.0;
        for w in words {
            local_sum += w.pickup_rate();
            if local_sum >= sample {
                return w.clone();
            }
        }
        return words[0].clone();
    }

    pub fn sample_vec(words : &Vec<Word>, count : usize) -> Vec<Word> {
        let mut res = vec![];
        for idx in 0..count {
            res.push(Word::sample(words));
        }
        res
    }
}
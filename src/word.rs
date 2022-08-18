
use serde::{Deserialize, Serialize};
use rand::Rng;
use log::*;

const PlayerELO : f32 = 1000.0;
const EloK : f32 = 128.0;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Word {
    pub ru_name : String,
    pub eng_name : String,
    w : f32,
    last_remind : std::time::SystemTime
}

impl Default for Word {
    fn default() -> Self {
        Self {
            ru_name : "".to_string(),
            eng_name : "".to_string(),
            w : PlayerELO,
            last_remind : std::time::SystemTime::now()
        }
    }
}

fn sigmoid(val : f32) -> f32 {
    1.0 / (1.0 + (-val).exp())
}

impl Word {

    pub fn get_elo(&self) -> f32 {
        self.w as f32
    }

    pub fn P(&self) -> f32 {
        let rating_diff = PlayerELO - self.get_elo();
        1.0 / (1.0 + 10_f32.powf(-rating_diff / 400.0))
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

        let sample = rnd.gen_range(0.0..=sum);

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

    pub fn score_up(&mut self) {
        let rating_diff = PlayerELO - self.get_elo();
        let e_word = 1.0 / (1.0 + 10_f32.powf(rating_diff / 400.0));
        let new_w = self.w - EloK * e_word;
        info!("Score up word {}:{}  {}->{}", self.ru_name, self.eng_name, self.w, new_w);
        self.w = new_w;
        self.last_remind = std::time::SystemTime::now();
    }

    pub fn score_down(&mut self) {
        let rating_diff = PlayerELO - self.get_elo();
        let e_word = 1.0 / (1.0 + 10_f32.powf(rating_diff / 400.0));
        let new_w = self.w + EloK * (1.0 - e_word);
        info!("Score down word {}:{}  {}->{}", self.ru_name, self.eng_name, self.w, new_w);
        self.w = new_w;
        self.last_remind = std::time::SystemTime::now();
    }
}
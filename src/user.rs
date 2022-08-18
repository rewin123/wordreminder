use serde::{Serialize, Deserialize};
use teloxide::{prelude::*};
use rand::Rng;
use log::*;

use crate::word::Word;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Language {
    Eng,
    Rus
}


impl Default for Language {
    fn default() -> Self {
        Language::Rus
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TestState {
    pub words : Vec<Word>,
    pub idx : i32,
    pub q_lang : Language,
    pub score : i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserState {
    Default,
    Testing(TestState),
    AddWord(Word)
}

impl Default for UserState {
    fn default() -> Self {
        UserState::Default
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id : i64,
    pub words : Vec<Word>,
    pub state : UserState
}

impl User {
    pub fn word_contains(&self, text : &str) -> bool {
        for w in &self.words {
            if w.ru_name == text || w.eng_name == text {
                return true;
            }
        }
        return false;
    }

    pub fn add_word(&mut self, word : Word) -> bool {
        if self.word_contains(&word.ru_name.as_str())
            || self.word_contains(&word.eng_name.as_str()) {
            return false;
        }
        
        self.words.push(word);

        return true;
    }

    pub fn prepare_test(&mut self, dur : i32) {
        let mut test = TestState::default();

        let mut w_sum = 0.0;
        for w in &self.words {
            w_sum += w.pickup_rate();
        }
        let mut rng = rand::thread_rng();

        let mut target_words = vec![];
        for i in 0..dur {
            let rnd_w = rng.gen_range(0.0..w_sum);
            let mut local_sum = 0.0;
            for w in &self.words {
                local_sum += w.pickup_rate();
                if local_sum >= rnd_w {
                    target_words.push(w.clone());
                    break;
                }
            }
        }

        test.idx = 0;
        test.words = target_words;

        self.state = UserState::Testing(test);

        info!("Prepaired test {:?}", self.state.clone());
    }

    pub fn get_word_idx(&self, word : Word) -> Option<usize> {
        for idx in 0..self.words.len() {
            let w = &self.words[idx];
            if w.ru_name == word.ru_name || w.eng_name == word.eng_name {
                return Some(idx);
            }
        }
        return None;
    }
}
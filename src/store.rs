use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};

/// Thread-safe, in-memory Database for Storing `Question` and `Answer`
#[derive(Clone, Debug)]
pub struct Store {
    /// Table of `QuestionId` and `Question`
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    /// Table of `AnswerID` and `Answer`
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    /// Creates a new `Store`
    pub fn new() -> Self {
        Self {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Returns a Hashmap a question from a file
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read question.json")
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::types::{question::{QuestionId, Question}, answer::{AnswerId, Answer}};


#[derive(Clone, Debug)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read question.json")
    }
}
use serde::{Deserialize, Serialize};

use super::question::QuestionId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct AnswerId(pub String);

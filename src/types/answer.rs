use serde::{Deserialize, Serialize};

use super::question::QuestionId;

/// Answer struct that is getting extracted
/// from query params
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Answer {
    /// `AnswerId` of the Answer
    pub id: AnswerId,
    /// String content of the Answer
    pub content: String,
    /// `QuestionId` of the corresponding `Question`
    pub question_id: QuestionId,
}

/// An tuple struct to capture String for the `Answer`'s ID
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct AnswerId(pub String);

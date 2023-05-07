use serde::{Deserialize, Serialize};

/// Question struct for storing question data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    /// ID of the `Question`
    pub id: QuestionId,
    /// Title of the `Question`
    pub title: String,
    /// Content of the `Question`
    pub content: String,
    /// (Optional) Tags related to the `Question`
    pub tags: Option<Vec<String>>,
}

/// Tuple struct for parsing the ID of the `Question`
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct QuestionId(pub i32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

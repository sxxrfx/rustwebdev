use handle_errors::Error;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    question::{NewQuestion, Question, QuestionId},
};

/// Postgres Database connection pool for storing Questions and Answers
#[derive(Clone, Debug)]
pub struct Store {
    // Postgres connection pool
    pub connection: PgPool,
}

impl Store {
    /// Creates a new `Store`
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(_) => panic!("Could't establish DB connection"),
        };
        Self {
            connection: db_pool,
        }
    }

    ///
    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, handle_errors::Error> {
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(_) => Err(Error::DatabaseQueryError),
        }
    }

    ///
    pub async fn add_question(
        &self,
        new_question: NewQuestion,
    ) -> Result<Question, handle_errors::Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags) VAULES ($1 $2 $3) RETURNING id, title, content, tags"
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await{
            Ok(question) => Ok(question),
            Err(_) => Err(Error::DatabaseQueryError),
        }
    }

    ///
    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
    ) -> Result<Question, handle_errors::Error> {
        match sqlx::query(
            "UPDATE questions
            SET title = $1, content = $2, tags = $3
            WHERE id = $4
            RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(_) => Err(Error::DatabaseQueryError),
        }
    }

    ///
    pub async fn delete_question(
        &self,
        question_id: i32,
    ) -> Result<bool, handle_errors::Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::DatabaseQueryError),
        }
    }

    ///
    pub async fn add_answer(
        &self,
        new_answer: NewAnswer,
    ) -> Result<Answer, handle_errors::Error> {
        match sqlx::query(
            "INSERT INTO questions (content, question_id) VAULES ($1 $2) RETURNING id, content, question_id"
        )
        .bind(new_answer.content)
        .bind(new_answer.question_id.0)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("tags")),
        })
        .fetch_one(&self.connection)
        .await{
            Ok(question) => Ok(question),
            Err(_) => Err(Error::DatabaseQueryError),
        }
    }
}

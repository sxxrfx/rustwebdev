use std::collections::HashMap;

use handle_errors::Error;
use warp::hyper::StatusCode;

use crate::{
    store::Store,
    types::{
        answer::{Answer, AnswerId},
        question::QuestionId,
    },
};

pub async fn add_answers(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params
            .get("content")
            .ok_or(Error::MissingParameters)?
            .to_string(),
        question_id: QuestionId(
            params
                .get("questionId")
                .ok_or(Error::MissingParameters)?
                .to_string(),
        ),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}

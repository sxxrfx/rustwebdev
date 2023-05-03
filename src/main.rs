#![allow(dead_code)]
#![allow(unused_variables)]
use std::{collections::HashMap, fmt, io::ErrorKind, str::FromStr};

use warp::{
    cors::CorsForbidden, http::Method, http::StatusCode, reject::Reject, Filter, Rejection, Reply,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
struct QuestionId(String);

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Self {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read question.json")
    }

    // fn add_question(mut self, question: Question) -> Self {
    //     self.questions.insert(question.id.clone(), question);
    //     self
    // }
}

// #[derive(Debug)]
// struct InvalidId;

// impl Reject for InvalidId {}

// impl Question {
//     pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
//         Self {
//             id,
//             title,
//             content,
//             tags,
//         }
//     }

//     fn update_title(&self, new_title: String) -> Self {
//         Question {
//             id: self.id.clone(),
//             title: new_title,
//             content: self.content.clone(),
//             tags: self.tags.clone(),
//         }
//     }
// }

// impl fmt::Display for Question {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}, title: {}. content: {}, tags: {:?}",
//             self.id, self.title, self.content, self.tags
//         )
//     }
// }

// impl fmt::Display for QuestionId {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "id : {}", self.0)
//     }
// }

// impl FromStr for QuestionId {
//     type Err = std::io::Error;

//     fn from_str(id: &str) -> Result<Self, Self::Err> {
//         match id.is_empty() {
//             false => Ok(QuestionId(id.to_string())),
//             true => Err(std::io::Error::new(
//                 ErrorKind::InvalidInput,
//                 "No id provided",
//             )),
//         }
//     }
// }

#[tokio::main]
async fn main() {
    let store = Store::new();

    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        // .allow_header("not-in-the-request")
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_questions);

    let routes = get_items.with(cors).recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn get_questions(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    // let question = Question::new(
    //     QuestionId::from_str("1").expect("No id provided"),
    //     "First Question".to_string(),
    //     "Content of Question".to_string(),
    //     Some(vec!["faq".to_string()]),
    // );

    // match question.id.0.parse::<i32>() {
    //     Ok(_) => Ok(warp::reply::json(&question)),
    //     Err(_) => Err(warp::reject::custom(InvalidId)),
    // }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(_) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

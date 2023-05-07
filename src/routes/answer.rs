use handle_errors::Error;
use warp::hyper::StatusCode;

use crate::{store::Store, types::answer::NewAnswer};

///
pub async fn add_answers(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Answer added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

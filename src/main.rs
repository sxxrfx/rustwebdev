use core::panic;
use std::env;

use clap::Parser;
use handle_errors::return_error;
use rustwebdev::{
    config::Config,
    routes::{self},
    store::Store,
};
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    let config = Config::new().expect("Config can't be set");

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rest_api={},warp={}",
            config.log_level, config.log_level, config.log_level
        )
    });

    let store = Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_port,
        config.db_name,
    ))
    .await
    .map_err(|e| handle_errors::Error::DatabaseQueryError(e))?;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .map_err(|e| handle_errors::Error::MigrationError(e))?;

    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::PUT,
            Method::DELETE,
            Method::GET,
            Method::POST,
        ]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::auth::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::auth::auth())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::auth::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let add_answers = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(routes::auth::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::answer::add_answers);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::auth::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::auth::login);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answers)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    tracing::info!(
        "Q&A service build ID {}",
        env!("RUST_WEB_DEV_VERSION")
    );
    // warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;
    Ok(())
}

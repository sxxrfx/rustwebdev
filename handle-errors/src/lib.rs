use core::fmt;

use tracing::{event, instrument, Level};
use warp::{
    body::BodyDeserializeError, cors::CorsForbidden, hyper::StatusCode,
    reject::Reject, Rejection, Reply,
};

use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as MiddlewareReqwestError;
use argon2::Error as ArgonError;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    IndexOutOfBound,
    WrongPassword,
    CannotDecryptToken,
    ArgonLibraryError(ArgonError),
    DatabaseQueryError(sqlx::Error),
    ReqwestAPIError(ReqwestError),
    MiddlewareReqwestAPIError(MiddlewareReqwestError),
    ClientError(APILayerError),
    ServerError(APILayerError),
    Unauthorized,
    MigrationError(sqlx::migrate::MigrateError),
}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ParseError(ref e) => {
                write!(f, "Cannot parse parameter: {}", e)
            }
            Error::MissingParameters => write!(f, "Missing parameter(s)"),
            Error::IndexOutOfBound => write!(f, "Index out of bound"),
            Error::DatabaseQueryError(_) => {
                write!(f, "Cannot update, invalid data.")
            }
            Error::ReqwestAPIError(ref err) => {
                write!(f, "External API error: {}", err)
            }
            Error::MiddlewareReqwestAPIError(ref err) => {
                write!(f, "External API error: {}", err)
            }
            Error::ClientError(ref err) => {
                write!(f, "External Client error: {}", err)
            }
            Error::ServerError(ref err) => {
                write!(f, "External Server error: {}", err)
            }
            Error::WrongPassword => {
                write!(f, "Wrong password")
            },
            Error::ArgonLibraryError(_) => {
                write!(f, "Cannot verify password")
            },
            Error::CannotDecryptToken => {
                write!(f, "Cannot decrypt auth token")
            },
            Error::Unauthorized => {
                write!(f, "No permission to change the underlying resource")
            },
            Error::MigrationError(_) => {
                write!(f, "Cannot migrate data")
            },
        }
    }
}

impl Reject for Error {}
impl Reject for APILayerError {}

const DUPLICATE_KEY_ERRORCODE: u32 = 23505;
#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(crate::Error::DatabaseQueryError(e)) = r.find() {
        event!(Level::ERROR, "Database query error");
        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY_ERRORCODE {
                Ok(warp::reply::with_status( "Account already exists".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
                } else {

            Ok(warp::reply::with_status(
                "Cannot update data".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,))
                }
            },
            _ => Ok(warp::reply::with_status(
                "Cannot update data".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            )),
        }
    } else if let Some(crate::Error::ReqwestAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::WrongPassword) = r.find() {
        event!(Level::ERROR, "Entered wrong password");
        Ok(warp::reply::with_status(
            "Wrong E-mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(crate::Error::ArgonLibraryError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Wrong E-mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(crate::Error::Unauthorized) = r.find() {
        event!(Level::ERROR, "Not matching account id");
        Ok(warp::reply::with_status(
            "No permitted to change underlying resource".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(crate::Error::MiddlewareReqwestAPIError(e)) =
        r.find()
    {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::ClientError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::ServerError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        event!(Level::ERROR, "CORS forbidden error: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        event!(Level::ERROR, "Cannot deserialize request body: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<Error>() {
        event!(Level::ERROR, "{}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

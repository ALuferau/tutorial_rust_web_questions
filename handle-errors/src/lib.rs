#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    InvalidRange,
    QuestionNotFound,
    WrongPassword,
    Unauthorized,
    TokenError,
    DatabaseQueryError(sqlx::Error),
    ExternalAPIError(reqwest::Error),
    ClientError(APILayerError),
    ServerError(APILayerError),
    ArgonLibraryError(argon2::Error),
}
#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ParseError(ref err) => write!(f, "Can't parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::InvalidRange => write!(f, "Invalid range"),
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::WrongPassword => write!(f, "Wrong Password"),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::TokenError => write!(f, "Token Error"),
            Error::DatabaseQueryError(_) => write!(f, "Query could not be executed"),
            Error::ExternalAPIError(ref err) => write!(f, "External api error: {}", err),
            Error::ClientError(ref err) => write!(f, "Client error: {}, status: {}", err.message, err.status),
            Error::ServerError(ref err) => write!(f, "Server error: {}, status: {}", err.message, err.status),
            Error::ArgonLibraryError(ref err) => write!(f, "Auth error: {}", err),
        }
    }
}
impl warp::reject::Reject for Error {}

#[derive(Debug)]
pub struct InvalidId;
impl warp::reject::Reject for InvalidId {}

const DUPLICATE_KEY: u32 = 23505;

pub async fn return_error(r: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = r.find::<warp::filters::cors::CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::hyper::StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<warp::body::BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::hyper::StatusCode::NOT_FOUND,
        ))
    } else if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
            warp::hyper::StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::MissingParameters) = r.find() {
        Ok(warp::reply::with_status(
            "Missing Parameters".to_string(),
            warp::hyper::StatusCode::EXPECTATION_FAILED,
        ))
    } else if let Some(Error::InvalidRange) = r.find() {
        Ok(warp::reply::with_status(
            "Invalid range".to_string(),
            warp::hyper::StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(Error::ParseError(err)) = r.find() {
        Ok(warp::reply::with_status(
            format!("Parse error: {}", err),
            warp::hyper::StatusCode::EXPECTATION_FAILED,
        ))
    } else if let Some(Error::QuestionNotFound) = r.find() {
        Ok(warp::reply::with_status(
            "Question not found".to_string(),
            warp::hyper::StatusCode::NOT_FOUND,
        ))
    } else if let Some(Error::WrongPassword) = r.find() {
        Ok(warp::reply::with_status(
            "Wrong E-Mail/Password combination".to_string(),
            warp::hyper::StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(Error::Unauthorized) = r.find() {
        Ok(warp::reply::with_status(
            "Unauthorized".to_string(),
            warp::hyper::StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(Error::TokenError) = r.find() {
        Ok(warp::reply::with_status(
            "Token Error".to_string(),
            warp::hyper::StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(Error::DatabaseQueryError(e)) = r.find() {
        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        "Account already exists".to_string(),
                        warp::hyper::StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Database Query Error".to_string(),
                        warp::hyper::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            },
            _ => {
                Ok(warp::reply::with_status(
                    "Database Query Error".to_string(),
                    warp::hyper::StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    } else if let Some(Error::ExternalAPIError(_)) = r.find() {
        Ok(warp::reply::with_status(
            "External api error".to_string(),
            warp::hyper::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ClientError(_)) = r.find() {
        Ok(warp::reply::with_status(
            "Internal server error".to_string(),
            warp::hyper::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ServerError(_)) = r.find() {
        Ok(warp::reply::with_status(
            "Internal server error".to_string(),
            warp::hyper::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::ArgonLibraryError(_)) = r.find() {
        Ok(warp::reply::with_status(
            "Internal server error".to_string(),
            warp::hyper::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }  else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            warp::hyper::StatusCode::NOT_FOUND,
        ))
    }
}

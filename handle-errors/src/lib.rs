#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    InvalidRange,
    QuestionNotFound,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Can't parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::InvalidRange => write!(f, "Invalid range"),
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}
impl warp::reject::Reject for Error {}

#[derive(Debug)]
pub struct InvalidId;
impl warp::reject::Reject for InvalidId {}

pub async fn return_error(r: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", r);
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
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            warp::hyper::StatusCode::NOT_FOUND,
        ))
    }
}

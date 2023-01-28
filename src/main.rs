use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, io::ErrorKind, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use warp::{
    body::BodyDeserializeError, filters::cors::CorsForbidden, http::Method, hyper::StatusCode,
    reject::Reject, Filter, Rejection,
};

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
struct QuestionId(String);
impl Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0,)
    }
}
impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "No id provided",
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let questions = store.questions.read().await;
    let mut start_pos = 0;
    let mut end_pos = 20;
    if !params.is_empty() {
        let pagination = get_pagination(params)?;
        start_pos = pagination.start;
        end_pos = pagination.end;
    }

    let res: Vec<&Question> = questions
        .values()
        .skip(start_pos)
        .take(end_pos - start_pos)
        .collect();

    Ok(warp::reply::json(&res))
}

async fn get_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let questions = store.questions.read().await;
    match questions.get(&QuestionId(id)) {
        Some(q) => Ok(warp::reply::json(&q)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status(
        "Question added",
        StatusCode::CREATED,
    ))
}

async fn update_question(
    id: String,
    store: Store,
    mut question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    question.id = QuestionId(id);
    match store.questions.write().await.get_mut(&question.id) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    };

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

async fn delete_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

fn get_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        Pagination::new(&params)
    } else {
        Err(Error::MissingParameters)
    }
}

async fn return_error(r: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::NOT_FOUND,
        ))
    } else if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::MissingParameters) = r.find() {
        Ok(warp::reply::with_status(
            "Missing Parameters".to_string(),
            StatusCode::EXPECTATION_FAILED,
        ))
    } else if let Some(Error::InvalidRange) = r.find() {
        Ok(warp::reply::with_status(
            "Invalid range".to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(Error::ParseError(err)) = r.find() {
        Ok(warp::reply::with_status(
            format!("Parse error: {}", err),
            StatusCode::EXPECTATION_FAILED,
        ))
    } else if let Some(Error::QuestionNotFound) = r.find() {
        Ok(warp::reply::with_status(
            "Question not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("not-in-the-request")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let question_path = warp::path("questions");

    let get_questions = warp::get()
        .and(question_path)
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let get_question = warp::put()
        .and(question_path)
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_question);

    let add_question = warp::post()
        .and(question_path)
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let update_question = warp::put()
        .and(question_path)
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(question_path)
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    let routes = get_questions
        .or(get_question)
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}
impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    InvalidRange,
    QuestionNotFound,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Can't parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::InvalidRange => write!(f, "Invalid range"),
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}
impl Reject for Error {}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}
impl Pagination {
    pub fn new(params: &HashMap<String, String>) -> Result<Self, Error> {
        let start = Pagination::get_value("start", &params)?;
        let end = Pagination::get_value("end", &params)?;
        if end >= start {
            Ok(Pagination {
                start: start,
                end: end,
            })
        } else {
            Err(Error::InvalidRange)
        }
    }
    fn get_value(key: &str, params: &HashMap<String, String>) -> Result<usize, Error> {
        params
            .get(key)
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)
    }
}

use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::Row;

use crate::types::question::{Question, QuestionId, NewQuestion};
use crate::types::answer::{Answer, AnswerId, NewAnswer};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}
impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url).await {
                Ok(pool) => pool,
                Err(_) => panic!("store::new Unable to connect to the Db"),
            };

        match sqlx::migrate!().run(&db_pool).await {
            Ok(res) => tracing::event!(tracing::Level::INFO, "store::migrated success {:?}", res),
            Err(e) => tracing::event!(tracing::Level::ERROR, "store::migrated error {:?}", e),
        }

        Store {
            connection: db_pool,
        }
    }
    pub async fn get_questions(&self, limit: Option<i32>, offset: i32) -> Result<Vec<Question>, handle_errors::Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(map_to_question)
            .fetch_all(&self.connection)
            .await {
                Ok(questions) => Ok(questions),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "store::get_questions {:?}", e);
                    Err(handle_errors::Error::DatabaseQueryError)
                }
            }
    }
    pub async fn get_question(&self, id: i32) -> Result<Option<Question>, handle_errors::Error> {
        match sqlx::query("SELECT * FROM questions WHERE id = $1")
            .bind(id)
            .map(map_to_question)
            .fetch_optional(&self.connection)
            .await {
                Ok(question) => Ok(question),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "store::get_question {:?}", e);
                    Err(handle_errors::Error::DatabaseQueryError)
                }
            }
    }
    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, handle_errors::Error> {
        match sqlx::query("INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3) RETURNING id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .map(map_to_question)
            .fetch_one(&self.connection)
            .await {
                Ok(question) => Ok(question),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "store::add_question {:?}", e);
                    Err(handle_errors::Error::DatabaseQueryError)
                },
            }
    }
    pub async fn update_question(&self, id: i32, question: Question) -> Result<Question, handle_errors::Error> {
        match sqlx::query("UPDATE questions SET title = $1, content = $2, tags = $3 WHERE id = $4 RETURNING id, title, content, tags")
            .bind(question.title)
            .bind(question.content)
            .bind(question.tags)
            .bind(id)
            .map(map_to_question)
            .fetch_one(&self.connection)
            .await {
                Ok(question) => Ok(question),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "store::update_question {:?}", e);
                    Err(handle_errors::Error::DatabaseQueryError)
                },
            }
    }
    pub async fn delete_question(&self, id: i32) -> Result<bool, handle_errors::Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await {
                Ok(_) => Ok(true),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "store::delete_question {:?}", e);
                    Err(handle_errors::Error::DatabaseQueryError)
                },
            }
    }

    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, handle_errors::Error> {
        match sqlx::query("INSERT INTO answers (content, question_id) VALUES ($1, $2)")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(map_to_answer)
            .fetch_one(&self.connection)
            .await {
                Ok(answer) => Ok(answer),
                Err(e) => {
                    tracing::event!(tracing::Level::ERROR, "store::add_answer {:?}", e);
                    Err(handle_errors::Error::DatabaseQueryError)
                },
            }
    }
}

fn map_to_question(row: PgRow) -> Question {
    Question {
        id: QuestionId(row.get("id")),
        title: row.get("title"),
        content: row.get("content"),
        tags: row.get("tags"),
    }
}

fn map_to_answer(row: PgRow) -> Answer {
    Answer {
        id: AnswerId(row.get("id")),
        content: row.get("content"),
        question_id: QuestionId(row.get("question_id")),
    }
}

// ref. from mongodb rust web development
// database crate
// postgresSQL connection pool
// Store structure
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::Row;

use crate::types::form::{
    Answer,
    Question, QuestionId,
/*     
// original example organises Question and Answer in separate files answer/question.rs
    answer::Answer,
    question::{Question, QuestionId}, */
};

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
            // ERROR: "formatting specifier missing"
            // when the original sample was a typo, [] invalid formatter, not {}
            Err(e) => panic!("Couldn't establish DB connection:{}", e),
         };

        Store {
            connection: db_pool,
        }
    }

// postgres=# insert into questions values (1, 'title', 'content', null, current_date);
pub async fn get_questions(
    &self,
    limit: Option<u32>,
    offset: u32
) -> Result<Vec<Question>, sqlx::Error> {
    //  Error: the relation 'questions' does not exist
    // https://bobcares.com/blog/postgresql-error-42p01/
    //match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
    match sqlx::query("SELECT * from questions")
      //  .bind(limit)
      //  .bind(offset)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
         })
    .fetch_all(&self.connection)
        .await {
            Ok(questions) => Ok(questions),
/*             Err(e) => {
            tracing::event!(tracing::Level::ERROR, "{:?}", e);
            Err(Error::DatabaseQueryError)
            } */
            Err(e) => Err(e),
        }
}
/*     fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    } */
}
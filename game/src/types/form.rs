// form/e-form related types
use serde::{Serialize, Deserialize};
use std::error::Error;
// note: support mongodb
// Document found in mongodb database collection
//use bson::document::Document;
   
/* CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
); 

CREATE TABLE IF NOT EXISTS answers (
   id serial PRIMARY KEY,
   content TEXT NOT NULL,
created_on TIMESTAMP NOT NULL DEFAULT NOW(),
   corresponding_question integer REFERENCES questions
);
*/

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Question {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: QuestionId,
   pub title: String,
   pub content: String,
   pub tags: Option<Vec<String>>,
}

// struct
// new type idiom
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub i32);

//use crate::types::form::QuestionId;
 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}
 
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);
use serde::{Serialize, Deserialize};
use std::error::Error;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Uuser {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<UuserId>,
   pub name: String,
   pub displayname: Option<String>,
   pub isactive: bool,
   pub tags: Option<Vec<Ulogin>>,
}

/// new type idiom
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UuserId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ulogin {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<i32>,
   pub name: UloginName,
   pub tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UloginName(pub String);
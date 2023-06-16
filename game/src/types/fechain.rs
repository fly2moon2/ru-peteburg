use serde::{Serialize, Deserialize};
use std::error::Error;
use sqlx::types::chrono::{DateTime, Utc};



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Uuser {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<UuserId>,
   pub name: String,
   pub display_name: Option<String>,
   pub isactive: bool,
   pub ulogin: Option<Vec<Ulogin>>,
}

/// new type idiom
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UuserId(pub i64);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ulogin {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<i64>,
   pub login: UloginLogin,
   pub start_dte: DateTime<Utc>,
   pub end_dte: DateTime<Utc>,
   pub frez_start_dte: DateTime<Utc>,
   pub frez_end_dte: DateTime<Utc>,
   pub uuser_id: i64,
   pub upasswd: Option<Vec<Upasswd>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UloginLogin(pub String);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Upasswd {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<i64>,
   pub pass: UpasswdPass,
   pub start_dte: DateTime<Utc>,
   pub end_dte: DateTime<Utc>,
   pub ulogin_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpasswdPass(pub String);
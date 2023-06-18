use serde::{Serialize, Deserialize};
use std::error::Error;
use sqlx::types::chrono::{DateTime, Utc};


/// users
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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UuserId(pub i64);

/// user types
/// standard - ordinary/normal users
/// admin - administrators/support
/// system - system-virtual user account
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UuserTyp {
    Standard,
    Admin,
    System,
}

/// user login/token
/// start/end date: valid period
/// friz_start/end_dte: any freeze/frozen (e.g. account locked within the valid period) 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ulogin {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<i64>,
   pub login: UloginLogin,
   pub typ: UloginTyp,
   pub start_dte: DateTime<Utc>,
   pub end_dte: DateTime<Utc>,
   pub friz_start_dte: DateTime<Utc>,
   pub friz_end_dte: DateTime<Utc>,
   pub uuser_id: i64,
   pub upasswd: Option<Vec<Upasswd>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UloginLogin(pub String);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UloginTyp {
    Login,
    LoginNToken,
    Token,
}

/// user password and validity
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Upasswd {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<i64>,
   pub pass: UpasswdPass,
   pub start_dte: DateTime<Utc>,
   pub end_dte: DateTime<Utc>,
   pub ulogin_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpasswdPass(pub String);

/// user role
/// parent_urole_id - points to the parent role (null for root level)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct URole {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<i64>,
   pub name: String,
   pub isactive: bool,
   pub parent_urole_id: Option<i64>,
}

/// mapping of user & roles
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UuserUrole {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<i64>,
   pub uuser_id: i64,
   pub urole_id: i64,
}

/// application resource
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Resauce {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<i64>,
   pub name: String,
   pub isactive: bool,
}

/// user role rights
/// start/end date: valid period
/// friz_start/end_dte: any freeze/frozen (e.g. account locked within the valid period) 
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Uright {
   //#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub id: Option<i64>,
   pub resauce_id: i64,
   pub urole_id: i64,
   pub right_typ: UrightTyp,
   pub start_dte: DateTime<Utc>,
   pub end_dte: DateTime<Utc>,
   pub friz_start_dte: DateTime<Utc>,
   pub friz_end_dte: DateTime<Utc>,
}

/// types of user role rights
/// ReadOrAccess - read only, open, general execution access
/// ReadWrite - read and write
/// other function-specific rights, with custom string as label
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UrightTyp {
    ReadOrAccess,
    ReadWrite,
    SpecificRight(String),
}
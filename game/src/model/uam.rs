use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}

// struct
// new type idiom
#[derive(Debug)]
pub struct Sex (String);

#[derive(Debug)]
pub struct Person {
  pub name: String,
  pub  sex: Sex,
  //sex: String,
  pub dob: String,
  pub data: Vec<u8>,
}

impl Person {
   pub fn new(name: &str) -> Person {
      Person {
         name: String::from(name),
         //sex: String::from("M"),
         sex: Sex(String::from("F")),
         dob: String::from("dob"),
         data: Vec::new(),
      }
   }
}



/* //#[derive(Debug)]
struct Appointment {
   client: Person,
   date: String,
   active_status: ActiveStatus,
}

impl Appointment {
   fn new(client: Person)->Appointment{
      Appointment {
         client: client,
         date: String::from("dates"),
         active_status: ActiveStatus::Active,
      }
   }
} */
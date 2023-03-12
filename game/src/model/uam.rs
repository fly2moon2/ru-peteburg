use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use std::error::Error;
// note: support mongodb
// Document found in mongodb database collection
use bson::document::Document;

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

// note: helper class to convert document to person object
// ref: https://blog.logrocket.com/using-mongodb-in-a-rust-web-service/
pub fn doc2Person(doc: &Document) -> Result<Person, Box<dyn Error>> {
   //fn doc_to_person(&self, doc: &Document) -> Result<Person, Box<dyn Error>> {
   
      
       //let id = doc.get_object_id(ID)?;
       //const NAME: &str = "name";
       let persname = doc.get_str("name")?;
   /*     let author = doc.get_str(AUTHOR)?;
       let num_pages = doc.get_i32(NUM_PAGES)?;
       let added_at = doc.get_datetime(ADDED_AT)?;
       let tags = doc.get_array(TAGS)?; */
   
       // note: 
       // person1 has to be mutable for subsequent updates of properties
       // String::from() converts from &str

       let mut person1=Person::new(persname);
       person1.dob=String::from(doc.get_str("dob")?);
   /*     let person = Person {
           name: persname.to_string(),
           sex: Sex(String::from("F")),
           dob: String::from("dob"),
           data: Vec::new(),
       }; */
       Ok(person1)
   /*     let book = Book {
           id: id.to_hex(),
           name: name.to_owned(),
           author: author.to_owned(),
           num_pages: num_pages as usize,
           added_at: *added_at,
           tags: tags
               .iter()
               .filter_map(|entry| match entry {
                   Bson::String(v) => Some(v.to_owned()),
                   _ => None,
               })
               .collect(),
       };
       Ok(book) */
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
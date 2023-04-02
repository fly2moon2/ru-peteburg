use mongodb::{Client, Collection, options::{ClientOptions, ResolverConfig}};
// Document found in mongodb database collection
use bson::document::Document;
use std::env;
use std::error::Error;
use std::time::Duration;
//use tokio;

pub async fn connect() -> Result<Client, Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
       env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
 
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let mut options =
       ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
          .await?;
          //options.app_name = Some("My App".to_string());
          //options.connect_timeout = Some(Duration::new(0, 20));
          //options.max_pool_size = Some(500u32);
          //options.min_pool_size = Some(10u32);
          //options.direct_connection = Some(true);
    let client = Client::with_options(options)?;
 
    // client implements std::sync::Arc, can use clone()
    // https://mongodb.github.io/mongo-rust-driver/manual/connecting.html
    //let client1 = client.clone();
 
/*     // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
       println!("- {}", name);
    }
 */
    //Ok(())
    Ok(client)
}


// note:    db get document collection
// returns a db collection from the given db connection client
pub async fn getcollection(client:&Client, dbname:String, collectionname:String) -> Result<Collection<Document>, Box<dyn Error>> {
   // return database collection
   let collection = client.database(&dbname).collection(&collectionname);

    Ok(collection)
}


pub async fn connectncollect(dbname:String, collectionname:String) -> Result<Collection<Document>, Box<dyn Error>> {
   // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let mut options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
         //options.app_name = Some("My App".to_string());
         options.connect_timeout = Some(Duration::new(0, 5));
         options.max_pool_size = Some(20u32);
         options.min_pool_size = Some(3u32);
         options.direct_connection = Some(true);
   let client = Client::with_options(options)?;
   let collection = client.database(&dbname).collection(&collectionname);

    Ok(collection)
}


//use mongodb::{bson::doc, options::ClientOptions, Client};
#[tokio::main]
pub async fn connectcld() -> mongodb::error::Result<()> {
        let client_options = ClientOptions::parse(
            "mongodb+srv://{user}:{pass}@crimeac0.6b1agmf.mongodb.net/?retryWrites=true&w=majority",
        )
        .await?;
        let client = Client::with_options(client_options)?;

        println!("altas cloud:Databases:");
        for name in client.list_database_names(None, None).await? {
           println!("- {}", name);
        }

        let database = client.database("crimea");



        Ok(())
    }
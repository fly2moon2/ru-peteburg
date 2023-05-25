#![allow(dead_code)]
#![allow(unused_imports, unused_variables, unused_doc_comments)]
// -------------------
// note: game macroquad 
// -------------------
use macroquad::prelude::*;
// note: warp
use warp::Filter;

//use macroquad_font_renderer::Fonts;
//use macroquad_tiled as tiled;
/* use macroquad::{

    experimental::{
        collections::storage,
        coroutines::start_coroutine,

    },
    ui,
}; */

// ----------------------------------
// note: game macroquad declares ends
// ----------------------------------

// -------------------
// note: db mongodb
// -------------------
// ref.:https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/
//
// start mongodb instance:
// at {local}}/dev/engine/db/mongodb/bin
// ./mongod --dbpath /Users/jack/dev/data/mongodb/crimea
// or ./mongod1 (custom startup script)
// connecting mongodb, default no password
// see with MongoDB Conmpass
// export MONGODB_URI='mongodb://localhost:27017/'
// cargo run

use mongodb::{Client, Collection, options::{ClientOptions, ResolverConfig}};
// Document found in mongodb database collection
use bson::document::Document;
use std::env;
use std::error::Error;
use tokio;

use chrono::{TimeZone, Utc};
use mongodb::bson::doc;
// -------------------------------
// note: db mongodb - declare ends
// -------------------------------

use crate::core::db::Store;
// note:    module call
use crate::core::docdb::connect_collectx;
pub mod core;
// note:    module call ends

//const NOTO_SANS: &[u8] = include_bytes!("../assets/fonts/NotoSans-Regular.ttf");

//use std::mem::size_of;

static B:[u8;4]=[34,56,123,23];
static C:[u8;7]=[3,33,54,231,34,45,98];


pub fn init() {
    let a:usize=42;
    let b:&[u8;4]=&B;
    let c:Box<[u8]>=Box::new(C);

    println!("a is: {}",a);
    println!("location: {:p}",&a);

// petetest
    // String - growable vector
    // &str - fix-sized string slice

    // convert from &str to String
    let hellostrslice:&str;
    hellostrslice="string slice tanjobi";
    println!("hellostrslice,{}",hellostrslice);
    let hellotext:String=String::from(hellostrslice);
    println!("hellotext, {}",hellotext);

    // TryFrom / TryInto
    // https://doc.rust-lang.org/stable/rust-by-example/conversion/try_from_try_into.html
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);
    
    impl TryFrom<i32> for EvenNumber {
        type Error = ();
    
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
    //
    // TryFrom / TryInto ends

    // convert to string
    use std::fmt;

    struct Circle {
        radius: i32
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    // convert to string ends

    // parse number (types with FromStr trait) to string
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
    // parse number ends

    // expressions
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
    // expressions end

    // literals
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    // literals end

// petetest ends
}

#[derive(Default)]
pub struct PlayerState {
    pub position: Vec2,
    pub rotation: f32,
}

pub struct Game {
    pub quit: bool,
    pub player_state: PlayerState,
    pub texture: Texture2D,
    pub texture2: Texture2D,
    pub game_speed:f32,
    pub opacity:f32
}

impl Game {
    pub async fn new() -> Self {
        let texture = load_texture("assets/plane.png").await.unwrap();
        let texture2 = load_texture("assets/starship1.png").await.unwrap();
        //
       


        Self {
            player_state: PlayerState {
                position: Vec2::new(100f32, 100f32),
                rotation: 0f32,
            },
            texture,
            texture2,
            quit: false,
            game_speed: 0.6f32,
            opacity: 1.0f32,
        }
    }



    pub fn update(&mut self) {
        if is_key_down(KeyCode::Escape) {
            self.quit = true;
        }
        const ROT_SPEED: f32 = 0.015;

        if is_key_down(KeyCode::Right) {
            self.player_state.rotation += ROT_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            self.player_state.rotation -= ROT_SPEED;
        }

        if is_key_down(KeyCode::Up) {
            self.game_speed = self.game_speed + 0.6;
        }
        if is_key_down(KeyCode::Down) {
            self.game_speed = self.game_speed - 0.2;
        }

        // add/minus opacity
        if is_key_down(KeyCode::A) && (self.opacity < 1.0) {
            self.opacity = self.opacity + 0.02;
        }
        if is_key_down(KeyCode::D) && (self.opacity > 0.0) {
            self.opacity = self.opacity - 0.02;
        }

        const SPEED: f32 = 0.6;
        

        //self.player_state.position += vec2_from_angle(self.player_state.rotation) * SPEED;
        self.player_state.position += vec2_from_angle(self.player_state.rotation) * self.game_speed;

        if self.player_state.position.x > screen_width() {
            self.player_state.position.x = -self.texture.width();
        } else if self.player_state.position.x < -self.texture.width() {
            self.player_state.position.x = screen_width();
        }

        if self.player_state.position.y > screen_height() {
            self.player_state.position.y = -self.texture.height();
        } else if self.player_state.position.y < -self.texture.height() {
            self.player_state.position.y = screen_height();
        }
    }

    pub fn draw(&self) {
        //clear_background(color_u8!(255, 255, 255, 255));

        // texture
        draw_texture_ex(
            self.texture,
            self.player_state.position.x,
            self.player_state.position.y,
            //WHITE,
            Color::new(1.00,1.00,1.00,self.opacity),
            DrawTextureParams {
                rotation: self.player_state.rotation,
                ..Default::default()
            },
        );

        //texture2
        draw_texture_ex(
            self.texture2,
            self.player_state.position.x+100f32,
            self.player_state.position.y+100f32,
            //WHITE,
            Color::new(1.00,1.00,1.00,self.opacity),
            DrawTextureParams {
                rotation: self.player_state.rotation,
                ..Default::default()
            },
        );

        draw_box(Vec2::new(self.player_state.position.x + 100f32, self.player_state.position.y + 50f32), Vec2::new(15f32, 12f32));
    

        draw_text_ex("st. petersburg", self.player_state.position.x + 150f32, 120.0, TextParams::default());

        
    }
}

pub fn vec2_from_angle(angle: f32) -> Vec2 {
    let angle = angle - std::f32::consts::FRAC_PI_2;
    Vec2::new(angle.cos(), angle.sin())
}

fn draw_box(pos: Vec2, size: Vec2) {
    let dimension = size * 2.;
    let upper_left = pos - size;

    draw_rectangle(upper_left.x, upper_left.y, dimension.x, dimension.y, BLACK);
}

//https://github.com/heroiclabs/fishgame-macroquad/blob/master/src/main.rs
/* struct Resources {

    tiled_map: tiled::Map,
    stone:Texture2D,

}
 */
/* impl Resources {
    async fn new() -> Result<Resources, macroquad::prelude::FileError> {
        let tileset = load_texture("assets/tileset.png").await?;
        tileset.set_filter(FilterMode::Nearest);

        let stone = load_texture("assets/stone.png").await?;
        stone.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/map.json").await.unwrap();
    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[("tileset.png", tileset), ("stone.png", stone)],
        &[],
    )
    .unwrap();

    Ok(Resources {

        tiled_map,
        stone,

    })
    }
} */

/* pub fn tiled(){

    let resources_loading = start_coroutine(async move {
        let resources = Resources::new().await.unwrap();
    });
    while resources_loading.is_done() == false {
    }
    let resources = storage::get::<Resources>();
    for object in &resources.tiled_map.layers["stone"].objects {
        scene::add_node(Decoration::new(
            vec2(object.world_x, object.world_y),
            object.gid.unwrap(),
        ));
    }
    drop(resources);

} */

// note: models/types
// mod
// https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5
// https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
use crate::types::uam::Sex;
use crate::types::uam::Person;
use crate::types::uam::doc2Person;
//use crate::model::code::ActiveStatus;
use crate::types::form::Question;
pub mod types; // declared in \types\mod.rs

//db
#[tokio::main]
async fn dbconnectx() -> Result<Store, sqlx::Error>{
/*     let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        "handle_errors=warn,practical_rust_book=warn,warp=warn".to_owned()
    }); */
 
    // if you need to add a username and password, 
    // the connection would look like:
    // "postgres://username:password@localhost:5432/rustwebdev"
    //let store = store::Store::new("postgres://localhost:5432/rustwebdev").await;
    // original sample code was static value; Store structure organised in separate store.rs
    // export to env variable instead of static value
    // export POSTGRESDB_URI=postgres://localhost:5432/rustwebdev
    // .expect is needed to output the string, instead of Result
    let store = Store::new(&env::var("POSTGRESDB_URI").expect("You must set the POSTGRESDB_URI environment var!")).await;
    // let store_filter = warp::any().map(move || store.clone());
 
    log::debug!("dbconnectx store...");
    Ok(store)
}

use sqlx::Row;
use std::collections::HashMap;

#[tokio::main]
//#[instrument]
pub async fn get_questions(
    //params: HashMap<String, String>,
    store: Store,
) -> Result<Vec<Question>, sqlx::Error> {
/*     event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    } */

    // Error: Option expected, not u32
    // use e.g. Some(3u32) to put as Options (limit to show up to 3 records, starting from record 1)
    match store
    .get_questions(Some(3u32), 0u32)
    .await
    {
    Ok(res) => Ok(res),
    Err(e) => Err(e),
    }

/*     let store1=store.get_questions(Some(0u32), 10u32).await;

    Ok(store1) */
}

// doc db
#[tokio::main]
// note: db collect ver x - get the document collection
async fn ddbcollect(dbname:String, collectname:String) -> Result<Collection<Document>, Box<dyn Error>> {

    let collect1=connect_collectx(dbname,collectname).await.unwrap();
    
    Ok(collect1)
}

#[tokio::main]
// note: db doc ver x - get the document
async fn ddbdoc(dbname:String, collectname:String) -> Result<Document, Box<dyn Error>> {

    let soldiers=connect_collectx(dbname,collectname).await.unwrap();
    
     let doc1: Document = soldiers.find_one(
            doc! {
                  "name": "stPetersburg"
            },
            None,
         )
         .await?
         .expect("Can't find the document."); 

    Ok(doc1)
}

#[tokio::main]
// note: db collect ver x - get the document collection
async fn find_soldier(collect1:&Collection<Document>) -> Result<Document, Box<dyn Error>> {

    let doc1: Document = collect1.find_one(
        doc! {
              "name": "stPetersburg"
        },
        None,
     )
     .await?
     .expect("Can't find the document."); 

    Ok(doc1)
}

#[tokio::main]
async fn warper() {
    // note: warp
    // create a path Filter
    let hello = warp::path("hello").map(|| format!("Hello, World!"));

    // start the server and pass the route filter to it
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}


// note: game engine macroquad
#[macroquad::main("game")]
async fn main() {

    pretty_env_logger::init();
/* use crate::core::env::get_player;
    let players=get_player();
    for (name, id) in &players {
        println!("{name:?} is id:{id}");
    }
    use crate::core::env::get_propss;
    let propss=get_propss();
    for (key, val) in &propss {
        println!("{key:?} value:{val:?}");
    } */


    use crate::core::env::{Locale, Localeex, RunEnvironment, Prop, PropSet, EnvPropKey, EnvPropPack};
    use crate::core::elements::{OnDataAvailStrategy};

    let mut env_prop_key1 = EnvPropKey::new_born("EnvPropPack theme".into(), None, None);

    let mut epp1_kv=HashMap::new();

    epp1_kv.insert(env_prop_key1.clone(),"EnvPropPack theme prop value".to_string());
    let mut env_prop_pack1 = EnvPropPack::new_born(epp1_kv, None, None);

    env_prop_pack1.get_prop(env_prop_key1);

    let mut epp2_kv=HashMap::new();
    let mut env_prop_key2 = EnvPropKey::new_born("city".into(), Some(RunEnvironment::DEV), Some(Localeex::CURRENT));
    let mut env_prop_key2a = EnvPropKey::new_born("city".into(), Some(RunEnvironment::DEV), Some(Localeex::CHINESE));
    let mut env_prop_key2b = EnvPropKey::new_born("city".into(), Some(RunEnvironment::DEV), Some(Localeex::JAPANESE));
    let mut env_prop_key2c = EnvPropKey::new_born("city".into(), Some(RunEnvironment::DEV), Some(Localeex::ENGLISH));

    epp2_kv.insert(env_prop_key2.clone(),"city of angels".to_string());
    epp2_kv.insert(env_prop_key2a.clone(),"天使之城".to_string());
    epp2_kv.insert(env_prop_key2b.clone(),"エンジェル・シティ".to_string());
    epp2_kv.insert(env_prop_key2c.clone(),"EN city of angels EN".to_string());
    let mut env_prop_pack2 = EnvPropPack::new_born(epp2_kv, Some(OnDataAvailStrategy::DEFAULT_ON_UNAVAIL), Some(OnDataAvailStrategy::DEFAULT_ON_UNAVAIL));

    let mut epp2_val = env_prop_pack2.get_prop_ex("city".to_string(),RunEnvironment::DEV, Localeex::ITALIAN, None, None).unwrap();

    println!("epp2_val: {:?}", epp2_val);

    let mut eppack3 = EnvPropPack::new_born_ex("layout".to_string(), RunEnvironment::DEV, Localeex::GERMAN, "GERMAN layout".to_string(), Some(OnDataAvailStrategy::DEFAULT_ON_UNAVAIL), Some(OnDataAvailStrategy::DEFAULT_ON_UNAVAIL));
    eppack3.key_vals.insert(EnvPropKey::new_born("layout".into(), Some(RunEnvironment::CURRENT), Some(Localeex::ENGLISH)),"CURRENT layout".to_string());
    let epp3_val = eppack3.get_prop_ex("layout".to_string(),RunEnvironment::DEV, Localeex::ENGLISH, None, None);

    if epp3_val.is_err() {
        println!("epp3_val Err: {:?}", epp3_val.err().unwrap());
    } else {
        println!("epp3_val Ok: {:?}", epp3_val.ok().unwrap());
    }

/*     let mut env_prop1 = EnvProp::new_with_key("label_login".to_string());
    env_prop1.join(Localeex::ENGLISH,"Login".to_string());
    env_prop1.join(Localeex::CURRENT,"Splogn".to_string()); */

 /*    let mut env_prop1 = EnvProp::new_born("label_login".to_string(),Some(OnDataAvailStrategy::DEFAULT_ON_UNAVAIL), Some(OnDataAvailStrategy::DEFAULT_ON_UNAVAIL),Some(Localeex::ENGLISH),Some("Login".to_string()));
    env_prop1.add_locale_prop(Localeex::SPANISH,"Spanish Logn".to_string());
    for (a_locale, a_prop_val) in &env_prop1.locale_props {
        println!("EnvProp locale: {a_locale:?}, prop_val:{a_prop_val:?}");
    }  

    let mut env_prop_set1 = EnvPropSet::new_born(RunEnvironment::DEV, Localeex::ENGLISH, OnDataAvailStrategy::ERR_ON_UNAVAIL, OnDataAvailStrategy::DEFAULT_ON_UNAVAIL, Some(env_prop1));
    // env_prop_set1.add_env_prop(env_prop1); */

    let mut props1 = PropSet::new();
    props1.join_with_raw_data("En".to_string(),"English".to_string());
    props1.join_with_raw_data("cH".to_string(),"Chinese".to_string());

    for (prop_key, prop_val) in &props1.props {
        println!("prop_key {prop_key:?}, prop_val:{prop_val:?}");
    }  

    // option way 1 - match
    let prop1_opt=props1.find("CH".to_string());

    match prop1_opt {
        Some(prop1)=>println!("found prop1 prop_key {0:?}, prop_val {1:?}",prop1.key,prop1.val),
        None => println!("Cannot found the property!"),
    }

    // option way 2 - if let
    // https://blog.logrocket.com/understanding-rust-option-results-enums/
    // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    let prop_opt_typ: Option<Prop>=None;    // must initialise before use
    let prop2_opt=props1.find("EN".to_string());

    if let Some(prop2) = prop_opt_typ {
        println!("Option get from if let prop1 prop_key {0:?}, prop_val {1:?}",prop2.key,prop2.val);
    }

    let mut props2 = PropSet::new();
    let prop2a = Prop::new("fR".to_string(),"French".to_string());
    let prop2b = Prop::new("gm".to_string(),"Germany".to_string());
    props2.join(prop2a);
    props2.join(prop2b);
    props2.set_locale(Locale{code:"jp".to_string(),description:"Japapnese".to_string()});
    props2.set_env(RunEnvironment::DEV);

    for (prop_key, prop_val) in &props2.props {
        println!("PROP2: locale {:?} env {:?} prop_key {prop_key:?}, prop_val:{prop_val:?}",props2.locale,props2.env);
    }  

    props2.remove_with_raw_data("gm".to_string().to_uppercase());

    for (prop_key, prop_val) in &props2.props {
        println!("PROP2 aft removed: prop_key {prop_key:?}, prop_val:{prop_val:?}");
    }  

    let locale_en = Locale::new(None,Some("Italian".to_string()));  // or put None for null description
    println!("LOCALE: code {:?} descr {:?}",locale_en.code, locale_en.description );
    //calls warper would pause to listen to localhost/hello path
    //warper();

    use crate::core::envfa::{test_json_stat_read};

    test_json_stat_read(String::from("./assets/missy.json"));

    //use std::rc::Rc;
    //let mut g_person=Rc::new(Person::new("persname"));
    let mut g_person=Person::new("persAname");
// ============================
// note: db (postgresSQL)
// ============================
    let dbstore=dbconnectx();

    let got_questions=get_questions(dbstore.unwrap()).unwrap();

    for got_question in &got_questions {
        println!("Question title {}, content {}", got_question.title, got_question.content);
    }

// ============================
// note: document db (mongodb) 
// ============================
//async fn main() -> Result<(), Box<dyn Error>> {
    //dbconnect();
    // note:
    // pass variables to closure
    // https://rust-unofficial.github.io/patterns/idioms/pass-var-to-closure.html
   // let closure_db={
/*         let dbconnect1=dbconnection().unwrap();
        let dbcollection1=dbcollection(&dbconnect1).unwrap();
        let dbdoc1=dbdocument(&dbcollection1).unwrap(); */

        // ***************************
        // way 1: ddbdoc: ok
        // ***************************
        let dbdoc1=ddbdoc(String::from("crimea"),String::from("soldier")).unwrap();

        let g_person=doc2Person(&dbdoc1).unwrap();
        // ***************************
        // way 1: ddbdoc ENDs
        // ***************************

        // ***************************
        // way 2: find_soldier: panic
        // ***************************

        // =====================
        // LEARN
        // calling from main here (since not from #tokio), remove await(), just unwrap()
        // =====================
/*         let dbcollect1=ddbcollect(String::from("crimea"),String::from("soldier")).unwrap();
    
        // LEARN: error not handled. panic
        let dbdoc1: Document = find_soldier(&dbcollect1).unwrap(); */

        // *******************************
        // way 2: find_soldier: panic ENDS
        // *******************************
        // LEARN: async
        // https://blog.logrocket.com/a-practical-guide-to-async-in-rust/
        //use tokio::task;
        // ref
        //let res= task::spawn_blocking(move ||analyze(&txt)).await?;
        //let dbdoc1: Document = task::spawn_blocking(move ||find_soldier(&dbcollect1)).await.unwrap().unwrap();
/*         let dbdoc1_opt: Option<Document> = dbcollect1.find_one(
               doc! {
                     "name": "stPetersburg"
               },
               None,
            )
            .await
            .expect("Can't find the document.");  */


/*         let dbdoc1: Document = dbcollect1.find_one(
            doc! {
                    "name": "stPetersburg"
            },
            None,
            )
            .await
            .expect("Can't find the document.");  */



/*             let dbdoc1: Document = match find_soldier(&dbcollect1) {
                Ok(doc1) => doc1,
                Err(error) => panic!("Problem find_soldier: {:?}", error),
            }; */

            // sample ref
/*             let greeting_file = match greeting_file_result {
                Ok(file) => file,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            }; */

/*         let doc1_init = doc! {
            "title": "Parasite",
            "year": 2020,
            "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
            "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
         }; */

/*          let doc1_init=doc!{};

        // use of match expression to get the result out of Option
        let dbdoc1_rslt = match dbdoc1_opt {
            Some(doc1) => doc1,
            //None => doc1_init,
            None => doc!{},
        }; */

        //let g_person=doc2Person(&dbdoc1_rslt).unwrap();
        //println!("g_person doc NAME: {}, DOB: {}", g_person.name, g_person.dob);

        //let dbdoc1=dbdocument(&dbcollection(&dbconnect1).unwrap()).unwrap();
        //let dbdoc1=dbcollectdocument(&dbconnect1).unwrap();
/*         let zsoldiers=getcollection(&dbconnect1,String::from("crimea"),String::from("soldier")).await.unwrap();
        
        let zsoldier: Option<Document> = zsoldiers.find_one(
            doc! {
                  "name": "stPetersburdg"
            },
            None,
         ).await.unwrap(); 
         println!("soldiers: {:?}", zsoldier);
 */
        //dbtask();
/*         let dbdocs2=connectncollect(String::from("crimea"),String::from("soldier")).await.unwrap();
        let zsoldier: Option<Document> = dbdocs2.find_one(
            doc! {
                  "name": "stPetersburdg"
            },
            None,
         ).await.unwrap(); 
         println!("soldiers: {:?}", zsoldier); */

        //connectcld(); 
        //g_person=g_person.clone();
        println!("g_person aft ddbdoc NAME: {}, DOB: {}", g_person.name, g_person.dob);
    //};


    let mut game = Game::new().await;
    //
    let rust_logo = load_texture("assets/plane.png").await.unwrap();

    //
    //init()
    init();
    //tiled();

    //static mut FONT_SIZE: f32 = 32.0;

    let font = load_ttf_font("./assets/fonts/RobotoCondensed-LightItalic.ttf").await.unwrap();
    // vertical fonts not supported
    //let font = load_ttf_font("./assets/fonts/NotoSansHK-Regular.otf").await.unwrap();
    loop {
        clear_background(LIGHTGRAY);
        
        draw_text_ex("Custom font size:", 120.0, 120.0, TextParams::default());
        let mut y = 20.0;

        //https://pullanswer.com/questions/font-sizes-different-between-wasm-native

/*         let textparams = TextParams {
            font,
            font_size: unsafe { FONT_SIZE as u16 },
            color: BLACK,
            ..Default::default()
        }; */

        for font_size in (30..100).step_by(20) {
            //let text = "abcdef";
            let text = &g_person.name;
            let params = TextParams {
                font,
                font_size,
                ..Default::default()
            };

            y += font_size as f32;
            draw_text_ex(text, 20.0, y, params);
        }

        draw_text_ex("Dynamic font scale:", 20.0, 400.0, TextParams::default());
        draw_text_ex(
            //"abcd",
            &g_person.name,
            20.0,
            450.0,
            TextParams {
                font_size: 50,
                font_scale: get_time().sin() as f32 / 2.0 + 1.0,
                ..Default::default()
            },
        );
/*         set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        }); */
        // Going 3d!

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);


        draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), rust_logo, WHITE);

        draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        draw_sphere(vec3(10., 10., 10.), 1., rust_logo, BLACK);

        draw_circle_lines(50.0, 50f32, 30f32, 0.5f32, GREEN);
        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);
        draw_text("In an unsafe world", 20.0, 30.0, 30.0, BLUE);
        // 3d example ends
        game.update();
        game.draw();
        if game.quit {
            // note:    game main()-> db main returns result 
            //          break loop instead of return
            return;
            //break;
        }
        next_frame().await
    }

    // note:    game main()-> db main rturns result 
    //          returns  Ok result
    //Ok(())
}

// ====================================
// ARCIVED CODE CORNER
// ====================================
//
// # db connection utilities
//
/* #[tokio::main]
// note: await.unwrap() requires [#tokio::main]
async fn dbconnection() -> Result<Client, Box<dyn Error>> {
    // -----------------------
    // note:    
    // returns a db client connection (called the db mod)
    // -----------------------
    //let client1 = connect().await.unwrap();
    let client1 = connectx().await.unwrap();
    Ok(client1)
}

#[tokio::main]
// note: Collection<Document> : Document is the generic type of Collection
async fn dbcollection(client1:&Client) -> Result<Collection<Document>, Box<dyn Error>> {
    // -----------------------
    // note:   
    // returns a db collection (of documents) given the db client connection
    // -----------------------
    let collection1=getcollection(&client1,String::from("crimea"),String::from("soldier")).await.unwrap();
    Ok(collection1)
}

#[tokio::main]
// note: 
async fn dbdocument(collection1:&Collection<Document>) -> Result<Document, Box<dyn Error>> {
    // -----------------------
    // note:   
    // 
    let doc1: Document = collection1.find_one(
            doc! {
                  "name": "stPetersburdg"
            },
            None,
         )
         .await?
         .expect("Can't find the document.");

    Ok(doc1)
}

#[tokio::main]
// note: 
async fn dbcollectdocument(client1:&Client) -> Result<Document, Box<dyn Error>> {
    // -----------------------
    // note:   
    // 

    //let collection1=getcollection(&client1,String::from("crimea"),String::from("soldier")).await.unwrap();


    let soldiers=getcollection(&client1,String::from("crimea"),String::from("soldier")).await.unwrap();
    //println!("soldiers: {:?}", soldiers);
    
     let doc1: Document = soldiers.find_one(
            doc! {
                  "name": "stPetersburdg"
            },
            None,
         )
         .await?
         .expect("Can't find the document."); 

    Ok(doc1)
} 

#[tokio::main]
// note:    game main()-> db main rturns result 
async fn dbtask() -> Result<(), Box<dyn Error>> {
    // -----------------------
    // note:    db connect 
    // returns a db client connection (called the db mod)
    // -----------------------
    let client1 = connect().await.unwrap();
 
    // Print the databases in our MongoDB cluster:
    println!("main:Databases:");
    for name in client1.list_database_names(None, None).await? {
       println!("- {}", name);
    }

    // connect database collection
   //let soldiers = client1.database("crimea").collection("soldier");
   let soldiers=getcollection(&client1,String::from("crimea"),String::from("soldier")).await.unwrap();
      // Insert some documents into the "mydb.books" collection.
   //soldiers.insert_many(docs, None).await?;

   // struct to doc
   // note: insert new doc
/*    //let person=Person::new("Arigato");
   let person=Person::new("stPetersburg");

   let doc_pers1 = doc! {
      "name": person.name,
   };
   
   let rslt1 = soldiers.insert_one(doc_pers1.clone(), None).await?;
 */

// find document
// Look up one document:
let soldier: Document = soldiers.find_one(
    doc! {
          "name": "stPetersburg"
    },
    None,
 ).await?
 .expect("Can't find the document.");
 //println!("solider stPetersburg: {}", soldier);
//let soldier1=solider;
 
//const NAME: &str = "name";
//let soldiername = soldier.get_str(NAME)?;
// println!("solider stPetersburg NAME: {}", soldiername);

   let doc2pers=doc2Person(&soldier).unwrap();
   //println!("solider doc NAME: {}, sex {}", doc2pers.name, doc2pers.sex.to_string)();
   println!("solider doc NAME: {}, DOB: {}", doc2pers.name, doc2pers.dob);

   //g_person=Rc::new(doc2pers);
   //let g_person=doc2pers.clone();
   let g_person=doc2Person(&soldier).unwrap();
   println!("g_person doc NAME: {}, DOB: {}", g_person.name, g_person.dob);
    // -------------------------
    // note:    db connect ends 
    // -------------------------
    // note:    game main()-> db main rturns result 
    //          returns Ok result
    Ok(())
}


*/

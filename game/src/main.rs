// -------------------
// note: game macroquad 
// -------------------
use macroquad::prelude::*;


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

// note:    module call
use crate::sys::db::connect;
use crate::sys::db::getcollection;
//use crate::sys::db::connectncollect;
use crate::sys::db::connectx;
use crate::sys::db::connect_collectx;
pub mod sys;
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

// note: model
// mod
// https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-rocket-version-ah5
// https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
use crate::model::uam::Sex;
use crate::model::uam::Person;
use crate::model::uam::doc2Person;
//use crate::model::code::ActiveStatus;
pub mod model; // declared in \model\mod.rs

#[tokio::main]
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
// note: db doc ver x - get the document
async fn dbdocx() -> Result<Document, Box<dyn Error>> {

    let soldiers=connect_collectx(String::from("crimea"),String::from("soldier")).await.unwrap();
    
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

   //gPerson=Rc::new(doc2pers);
   //let gPerson=doc2pers.clone();
   let gPerson=doc2Person(&soldier).unwrap();
   println!("gPerson doc NAME: {}, DOB: {}", gPerson.name, gPerson.dob);
    // -------------------------
    // note:    db connect ends 
    // -------------------------
    // note:    game main()-> db main rturns result 
    //          returns Ok result
    Ok(())
}



// note: game engine macroquad
#[macroquad::main("game")]
async fn main() {
    // note:
    // pass variables to closure
    // https://rust-unofficial.github.io/patterns/idioms/pass-var-to-closure.html

    //use std::rc::Rc;
    //let mut gPerson=Rc::new(Person::new("persname"));
    let mut gPerson=Person::new("persAname");
//async fn main() -> Result<(), Box<dyn Error>> {
    //dbconnect();
   // let closure_db={
/*         let dbconnect1=dbconnection().unwrap();
        let dbcollection1=dbcollection(&dbconnect1).unwrap();
        let dbdoc1=dbdocument(&dbcollection1).unwrap(); */

        let dbdoc1=dbdocx().unwrap();
        let gPerson=doc2Person(&dbdoc1).unwrap();
        //println!("gPerson doc NAME: {}, DOB: {}", gPerson.name, gPerson.dob);

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
        //gPerson=gPerson.clone();
        println!("gPerson aft dbdocx NAME: {}, DOB: {}", gPerson.name, gPerson.dob);
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
            let text = &gPerson.name;
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
            "abcd",
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
    //          returns Ok result
    //Ok(())
}

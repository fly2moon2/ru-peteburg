// https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
/* Declaring modules: In the crate root file, you can declare new modules; say, you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
Inline, within curly brackets that replace the semicolon following mod garden
In the file src/garden.rs
In the file src/garden/mod.rs */
// to declare all sub-modules in this folder
//
// db module
pub mod db;
// document db [mongodb]
pub mod docdb;
// web socket [tungstenite]
pub mod wsock;
// environment variables/properties
pub mod env;
// carbona variables/properties
pub mod carbona;
// environment features advanced
pub mod envfa;
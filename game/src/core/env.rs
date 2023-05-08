use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use crate::core::element::GeneralPosSymbol;

// ========================================================================
// HashMap example
// Prop - application property/environment variable - key, value construct
// Props - locale (EN), Running Environment (DEV/PROD)
// and properties (a HashMap of Prop)
// ========================================================================

#[derive(Debug, Clone)]
pub enum RunEnvironment {
    DEV,
    UAT,
    SIT,
    PROD{label:String}, // label is used when there is a need to refine meaning of PRODuction env. (e.g. site01, DR, etc.)
    UNDEFINED,
}

/// Locale
/// CURRENT -   is special, meaning the current/default locale;
///             when given locale cannot be found for a property (EnvProp), 
///             default property value would be used (subject to locale_strategy in app_properties.json)
/// ref. https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust
/// hash, partialeq, eq, serialisze, deserialize is needed for putting the struct inside hashmap
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Localeex {
    CURRENT,
    ENGLISH,
    CHINESE,
    JAPANESE,
    FRENCH,
    ITALIAN,
    GERMAN,
}

impl Localeex{
    pub fn value(&self) -> String {
        match *self {
            Localeex::CURRENT => GeneralPosSymbol::Current.value(),
            Localeex::ENGLISH => "EN".to_string(),
            Localeex::CHINESE => "CH".to_string(),
            Localeex::JAPANESE => "JP".to_string(),
            Localeex::FRENCH => "FR".to_string(),
            Localeex::ITALIAN => "IT".to_string(),
            Localeex::GERMAN => "GR".to_string(),
        }
    }
}

/// hash, partialeq, eq, serialisze, deserialize is needed for putting the struct inside hashmap
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Locale {
    pub code: String,
    pub description: String,
}

impl Locale {
    // default to "." (current), if code is not given
    // description is optional
    pub fn new(code_opt: Option<String>, descr_opt: Option<String>) -> Locale {
        let some_code=code_opt.unwrap_or(GeneralPosSymbol::Current.value());
        let some_descr=descr_opt.unwrap_or("".to_string());
/*         let mut some_descr:String="".to_string();
        if let Some(descr)=descr_opt {
            some_descr=descr;
        } */
/*         match descr_opt {
            Some(descr1)=>some_descr=descr1,
            None => some_descr="".to_string(),
        } */
        Locale { code: some_code, description: some_descr }
    }
}

/// EnvProp
/// is next generation of Prop - environment properties
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EnvProp {
    pub key: String,
    pub locale_props:HashMap<Locale, String>,
}

impl EnvProp {
    pub fn new() -> EnvProp {
        EnvProp {
            key: String::from(""),
            locale_props: HashMap::new(),
        }
    }

    pub fn new_with_key(a_key: String) -> EnvProp {
        EnvProp {
            key: a_key,
            locale_props: HashMap::new(),
        }
    }

    pub fn join(
            &mut self, // must be mutable
            a_locale: Locale,
            a_prop_val: String,
        ) {
            self.locale_props.insert(a_locale, a_prop_val);
        }
}



/// EnvPropSet
/// EnvProp set
#[derive(Debug,Clone)]
pub struct EnvPropSet {
    pub env:RunEnvironment, 
    pub env_props:Vec<EnvProp>,
}

impl EnvPropSet {
    pub fn new() -> EnvPropSet {
        EnvPropSet {
            env: RunEnvironment::DEV,
            // properties collection defined as HashMap, instead of struct Prop
            // to take advantage of unique key mechanism
            env_props: Vec::new(),
        }
    }
}   


/// Prop
/// vanilla version of properties
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Prop {
    pub key: String,
    pub val: String,
}

impl Prop {
    pub fn new(key: String, val: String) -> Prop {
        Prop { key: key, val: val }
    }
}

/// PropSet: 
// HashMap of Prop (application properties) as well as locale and/or environment to define the context
/// 1st element is the (K)ey, 2nd element is the (V)alue
/// HashMap implementation prevents duplicate key
#[derive(Debug,Clone)]
pub struct PropSet {
    pub locale:Locale,
    pub env:RunEnvironment, 
    pub props:HashMap<String, String>,
}

impl PropSet {
    pub fn new() -> PropSet {
        PropSet {
            locale:Locale::new(None,None),
            env:RunEnvironment::UNDEFINED,
            // properties collection defined as HashMap, instead of struct Prop
            // to take advantage of unique key mechanism
            props: HashMap::new(),
        }
    }

    pub fn new_with_keyset(locale_opt:Option<Locale>, env_opt:Option<RunEnvironment>) -> PropSet {
        let mut some_locale:Locale;
        let mut some_env:RunEnvironment;
        match locale_opt {
            Some(locale1)=>some_locale=locale1,
            None => some_locale=Locale::new(None,None),
        }
        match env_opt {
            Some(env1)=>some_env=env1,
            None => some_env=RunEnvironment::DEV,
        }
        PropSet {
            locale:some_locale,
            env:some_env,
            // properties collection defined as HashMap, instead of struct Prop
            // to take advantage of unique key mechanism
            props: HashMap::new(),
        }
    }

    // set the locale
    pub fn set_locale(
        &mut self, // must be mutable
        locale: Locale,
    ) {
        self.locale=locale;
    }

    // set the environment
    pub fn set_env(
        &mut self, // must be mutable
        env: RunEnvironment,
    ) {
        self.env=env;
    }

    // a Property joins as member
    pub fn join(
        &mut self, // must be mutable
        prop: Prop,
    ) {
        self.join_with_raw_data(prop.key, prop.val);
    }

    // property in the form of JSON untyped value joins as member
    pub fn join_from_json_untyp(
        &mut self, // must be mutable
        json_untyp: Value,
    ) {
        self.join_with_raw_data(json_untyp["key"].to_string(),json_untyp["val"].to_string());
    }

    // same use as join, accepts raw data (key,value pair in String) for common usage
    pub fn join_with_raw_data(
        &mut self, // must be mutable
        prop_key: String,
        prop_val: String,
    ) {
        // force each property key in uppercase to ensure subsequenty matching is exact
        self.props.insert(prop_key.to_uppercase(), prop_val);
    }

    // removes a Property from the collection
    pub fn remove(
        &mut self, // must be mutable
        prop: Prop,
    ) {
        self.remove_with_raw_data(prop.key);
    }

    // same use as remove, accepts raw data (key in String) for common usage
    pub fn remove_with_raw_data(
        &mut self, // must be mutable
        prop_key: String,
    ) {
        self.props.remove(&prop_key);
    }

    // returns a Property when a matching key is found
    pub fn find(
        &self,
        prop_key: String,
    ) -> Option<Prop> {
        match &self.props.get(&prop_key) {
            //Some(prop_val) => println!("{prop_key}: {prop_val}"),
            Some(prop_val) => Some(Prop::new(prop_key.to_string(), prop_val.to_string())),
            //None => println!("{prop_key} is not found.")
            None => None,
        }
    }
}

// testing hashmap
/* pub struct PropSet  {
    pub locale: String,
    pub props: HashMap<String, Prop>, // the hash map owns the struct
}

impl PropSet {
    pub fn new() -> PropSet {
        PropSet {
            locale: "EN".to_string(),  // locale as the key; (EN)glish by default
            props: HashMap::new(),
        }
    }

    /// Join is used to add a new Prop into the hashmap
    pub fn join(
        &mut self, // must be mutable
        locale: &str,  
        prop: Prop,
    ) {
        // do not pass a reference
        self.props.insert(locale.to_string(), prop); // inserting moves `prop`
    }
}

pub fn test_prop_set() -> PropSet {

    let mut prop_set = PropSet::new();
  
    let prop1 = Prop::new("DB_URI".to_string(),"//path".to_string());

    prop_set.join("EN", prop1); 

    let prop2 = Prop::new("DOCDB_URI".to_string(),"//path/doc".to_string());

    prop_set.join("EN", prop2); 
  
    prop_set
  
  } */
// ========================================================================
// Prop, PropSet ENDs
// ========================================================================

/* pub fn get_props()->HashMap<String, Prop>{
    // type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut props_map = HashMap::new();
    
    /* fn random_stat_buff() -> u8 {
        // could actually return some random value here - let's just return
        // some fixed value for now
        42
    } */
    
    // insert a key only if it doesn't already exist
    //player_stats.entry("health").or_insert(100);
    
    props_map.entry("DB_URI".to_string()).or_insert("//path".to_string());
    props_map.entry("DB_URI".to_string()).or_insert("//path2".to_string());
    props_map.entry("DB_URI2".to_string()).or_insert("//path".to_string());
    

    /* // Use a HashMap to store the vikings' health points.
let vikings = HashMap::from([
    (Viking::new("Einar", "Norway"), 25),
    (Viking::new("Olaf", "Denmark"), 24),
    (Viking::new("Harald", "Iceland"), 12),
]); */

    // insert a key using a function that provides a new value only if it
    // doesn't already exist
    /* player_stats.entry("defence").or_insert_with(random_stat_buff); */
    let mut props:HashMap<Prop>=HashMap::new(); 

    for (key, value) in &props_map {
       props.insert(key,value);
    }

    props

} */

/* pub fn get_propss()->HashMap<String,String>{
    // type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example)(i.e. let mut props:HashMap<&str,u8>=HashMap::new();)
    let mut props = HashMap::new();
    
    /* fn random_stat_buff() -> u8 {
        // could actually return some random value here - let's just return
        // some fixed value for now
        42
    } */
    
    // insert a key only if it doesn't already exist
    //player_stats.entry("health").or_insert(100);
    
    props.entry("DB_URI".to_string()).or_insert("//path".to_string());
    props.entry("DB_URI".to_string()).or_insert("//path2".to_string());
    props.entry("DB_URI2".to_string()).or_insert("//path".to_string());
    
    // insert a key using a function that provides a new value only if it
    // doesn't already exist
    /* player_stats.entry("defence").or_insert_with(random_stat_buff); */
    
    props
}

pub fn get_player()->HashMap<String,u8>{
// type inference lets us omit an explicit type signature (which
// would be `HashMap<&str, u8>` in this example).
let mut player_stats = HashMap::new();

/* fn random_stat_buff() -> u8 {
    // could actually return some random value here - let's just return
    // some fixed value for now
    42
} */

// insert a key only if it doesn't already exist
//player_stats.entry("health").or_insert(100);

player_stats.entry("health".to_string()).or_insert(100);
player_stats.entry("health".to_string()).or_insert(101);
player_stats.entry("health2".to_string()).or_insert(100);

// insert a key using a function that provides a new value only if it
// doesn't already exist
/* player_stats.entry("defence").or_insert_with(random_stat_buff); */

player_stats
} */


/* 
Q: HaspMap Struct
https://stackoverflow.com/questions/54039307/creation-of-a-hashmap-with-struct-in-rust

A1)
use std::collections::HashMap;

#[derive(Clone)] // we'll be cloning it later on
struct Node<'a> {
    data: &'a i32 
}


struct Test<'a> {
    hash_map: HashMap<&'a str, Node<'a>>  // the hash map owns the struct
}

impl<'a> Test<'a> {
    fn new() -> Test<'a> {
        Test {hash_map: HashMap::new()}
    }

    fn join(
        &mut self, // must be mutable
        node: Node<'a>) { // do not pass a reference
        self.hash_map.insert("test", node);  // inserting moves `node`
    }
}

fn main() {
    let stuff = Node {data: &12};
    let mut test = Test::new();

    test.join(stuff.clone());  // if we don't clone, `stuff` will get moved

    println!("{}", *test.hash_map["test"].data);  // outputs "12"
}

Since std::collections::HashMap::insert attempts to move its second argument, one can't dereference a pointer to something and pass that to this method because otherwise the pointer will become uninitialized, which isn't permitted. A way so solve this is to pass a moved value and not a pointer to join.

A2)
For poor idiots like myself, who are trying to find out how to put hashmaps in a struct, no need to spend many hours "playing" with lifetimes(the 'a in the above example). They are not required in the slightest, just use String instead of &str in your structure.

struct ComputerConfig {
    hostname: String, 
    // displays: Vec<DispConfig>,
}

struct MyConfig {
    pub config_version: u8,
    computers: HashMap<String, ComputerConfig>, // the hash map owns the struct
}

impl MyConfig {
    fn new() -> MyConfig {
        MyConfig {
            computers: HashMap::new(),
            config_version: 1,
        }
    }

    /// Join is used to add a new ComputerConfig into the hashmap
    fn join(
        &mut self, // must be mutable
        key: &str,
        node: ComputerConfig,
    ) {
        // do not pass a reference
        self.computers.insert(key.to_string(), node); // inserting moves `node`
    }
}

fn main() {

  let mut cfg = MyConfig::new()

  cfg.join("test", stuff); 

  println!("{:?}", &cfg); // outputs "12"

}

     */
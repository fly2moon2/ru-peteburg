use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use crate::core::elements::{GeneralSymbol, OnDataAvailStrategy};

// ========================================================================
// HashMap example
// Prop - application property/environment variable - key, value construct
// Props - locale (EN), Running Environment (DEV/PROD)
// and properties (a HashMap of Prop)
// ========================================================================

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunEnvironment {
    CURRENT,
    DEV,
    UAT,
    SIT,
    PROD{label:String}, // label is used when there is a need to refine meaning of PRODuction env. (e.g. site01, DR, etc.)
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
    SPANISH,
}

impl Localeex{
/*     pub fn new_born(code:&str) -> Localeex {
        match code {
            GeneralSymbol::CURRENT.symbol() => Localeex::CURRENT => GeneralSymbol::CURRENT.symbol(),
            "EN".to_string() => Localeex::ENGLISH,
            "CH".to_string() => Localeex::CHINESE,
            "JP".to_string() => Localeex::JAPANESE,
            "FR".to_string() => Localeex::FRENCH,
            "IT".to_string() => Localeex::ITALIAN,
            "GR".to_string()=> Localeex::GERMAN,
            "SP".to_string() => Localeex::SPANISH,      
        }
    } */


    pub fn code(&self) -> String {
        match *self {
            Localeex::CURRENT => GeneralSymbol::CURRENT.symbol(),
            Localeex::ENGLISH => "EN".to_string(),
            Localeex::CHINESE => "CH".to_string(),
            Localeex::JAPANESE => "JP".to_string(),
            Localeex::FRENCH => "FR".to_string(),
            Localeex::ITALIAN => "IT".to_string(),
            Localeex::GERMAN => "GR".to_string(),
            Localeex::SPANISH => "SP".to_string(),
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
        let some_code=code_opt.unwrap_or(GeneralSymbol::CURRENT.symbol());
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




/// EnvPropPack
/// one or many environment properties, by running environment & locale

/// EnvPropKey
/// compound key of an environment property
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvPropKey {
    pub prop_key: String,
    pub run_env: RunEnvironment,
    pub locale: Localeex,
}

impl EnvPropKey {
/*     /// vanilla new
    ///  only key is mandatory, whereas the remaining properties are default
    pub fn new(a_key: String) -> EnvProp {
        EnvPropKey {
            prop_key: a_key,
            run_env: RunEnvironment::CURRENT,
            locale: Localeex::CURRENT,
        }
    } */
    
    pub fn new_born(a_key: String, a_run_env: Option<RunEnvironment>, a_locale: Option<Localeex>) -> EnvPropKey {
        EnvPropKey {
            prop_key: a_key,
            run_env: if a_run_env.is_none() {a_run_env.unwrap()} else {RunEnvironment::CURRENT},
            locale: if a_locale.is_none() {a_locale.unwrap()} else {Localeex::CURRENT},
        }
    }
}

/* #[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvPropVal (String); */

// type alias
type EnvPropVal=String;

//#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvPropPack {
    pub key_vals:HashMap<EnvPropKey, EnvPropVal>,
    pub run_env_strategy:OnDataAvailStrategy,
    pub locale_strategy:OnDataAvailStrategy,
}

impl EnvPropPack {
    pub fn new_born(a_key_vals: HashMap<EnvPropKey, EnvPropVal>, a_re_stgy: Option<OnDataAvailStrategy>, a_loc_stgy: Option<OnDataAvailStrategy>) -> EnvPropPack {
        EnvPropPack {
            key_vals: a_key_vals,
            run_env_strategy: if a_re_stgy.is_none() {OnDataAvailStrategy::INHERIT} else {a_re_stgy.unwrap()},
            locale_strategy: if a_loc_stgy.is_none() {OnDataAvailStrategy::INHERIT} else {a_loc_stgy.unwrap()},
        }
    }

    // returns a Property when a matching key is found
    pub fn get_val(&self, a_key: EnvPropKey) {
    //pub fn get_val(&self, a_key: EnvPropKey) -> Option<EnvPropVal> {
        match &self.key_vals.get(&a_key) {
            Some(a_val) => println!("{:?}: {a_val}",a_key.prop_key),
            //Some(a_val) => Some(a_val),
            None => println!("{:?} is not found.",a_key.prop_key)
            //None => None,
        }
    }

/*     pub fn get_prop(&self, a_cur_run_env: RunEnvironment, a_cur_locale: Localeex, a_parent_re_stgy: Option<OnDataAvailStrategy>, a_parent_loc_stgy: Option<OnDataAvailStrategy>) -> String {
        if self.env_prop_key.run_env==a_cur_run_env {
            if self.env_prop_key.locale==a_cur_locale {

            } else {

            }
        } else {

        }
        
        self.env_prop_val
    } */

/*     /// add to & delete from locale properties
    pub fn add_locale_prop(
        &mut self, // must be mutable
        a_loc: Localeex,
        a_prop_val: String,
    ) {
        self.locale_props.insert(a_loc, a_prop_val);
    }

    pub fn del_locale_prop(
        &mut self, // must be mutable
        a_loc: Localeex,
    ) {
        self.locale_props.remove(&a_loc);
    } */
}


/* 
/// EnvPropSet
/// EnvProp set
#[derive(Debug,Clone)]
pub struct EnvPropSet {
    pub run_env:RunEnvironment, 
    pub locale:Localeex,
    pub run_env_strategy:OnDataAvailStrategy,
    pub locale_strategy:OnDataAvailStrategy,
    /// key (property key of EnvProp) is string to facilitate record searching, 
    /// followed by the whole EnvProp object
    pub env_props:HashMap<EnvPropKey, EnvProp>,
}

impl EnvPropSet {
    /// vanilla new
    /// only running environment is mandatory, whereas the remaining properties are default
    pub fn new(a_run_env: RunEnvironment) -> EnvPropSet {
        EnvPropSet {
            run_env: a_run_env,
            default_locale: Localeex::CURRENT,
            run_env_strategy: OnDataAvailStrategy::DEFAULT_ON_UNAVAIL,
            locale_strategy: OnDataAvailStrategy::DEFAULT_ON_UNAVAIL,
            env_props: HashMap::new(),
        }
    }

    /// new & fill data at one go
    /// all arguments except env property are mandatory
    pub fn new_born(a_run_env: RunEnvironment, a_def_locale:Localeex, a_run_env_strategy:OnDataAvailStrategy, a_locale_strategy:OnDataAvailStrategy, a_env_prop:Option<EnvProp>) -> EnvPropSet {
        let mut o_env_prop = EnvPropSet {
            run_env: a_run_env,
            default_locale: a_def_locale,
            run_env_strategy: a_run_env_strategy,
            locale_strategy: a_locale_strategy,
            env_props: HashMap::new(),
        };
        if !a_env_prop.is_none() {
            o_env_prop.add_env_prop(a_env_prop.clone().unwrap().key, a_env_prop.unwrap());
        }
        o_env_prop
    }

    /// add to & delete from an EnvProp to env_props
    pub fn add_env_prop(
        &mut self, // must be mutable
        a_env_prop_key: String,
        a_env_prop: EnvProp,
    ) {
        self.env_props.insert(a_env_prop_key, a_env_prop);
    }

    pub fn del_env_prop(
        &mut self, // must be mutable
        a_env_prop_key: String,
    ) {
        self.env_props.remove(&a_env_prop_key);
    }

    // returns an env property when a matching key is found
    pub fn get_env_prop(
        &self,
        a_env_prop_key: String,
    ) -> Option<EnvProp> {
        match &self.env_props.get(&a_env_prop_key) {
            //Some(prop_val) => println!("{prop_key}: {prop_val}"),
            Some(o_env_prop) => Some(EnvProp::new(a_env_prop_key.to_string(), prop_val.to_string())),
/*             key: a_key,
            run_env_strategy:OnDataAvailStrategy::INHERIT,
            locale_strategy:OnDataAvailStrategy::INHERIT,
            locale_props: HashMap::new(), */
            //None => println!("{prop_key} is not found.")
            None => None,
        }
    }
}   
 */

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
/// 1st carbona is the (K)ey, 2nd carbona is the (V)alue
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
            env:RunEnvironment::CURRENT,
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
use std::collections::HashMap;
//use std::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use crate::core::elements::{GeneralSymbol, OnDataAvailStrategy, ErrorOnthefly};

// ========================================================================
// HashMap example
// Prop - application property/environment variable - key, value construct
// Props - locale (EN), Running Environment (DEV/PROD)
// and properties (a HashMap of Prop)
// ========================================================================

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunEnvironment {
    Current,
    DEV,
    UAT,
    SIT,
    PROD(String), // label is used when there is a need to refine meaning of PRODuction env. (e.g. site01, DR, etc.)
}

/// Locale
/// Current -   is special, meaning the current/default locale;
///             when given locale cannot be found for a property (EnvProp), 
///             default property value would be used (subject to locale_strategy in app_properties.json)
/// ref. https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust
/// hash, partialeq, eq, serialisze, deserialize is needed for putting the struct inside hashmap
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Localeex {
    Current,
    English,
    Chinese,
    Japanese,
    French,
    Italian,
    German,
    Spanish,
}

impl Localeex{
/*     pub fn new_born(code:&str) -> Localeex {
        match code {
            GeneralSymbol::Current.symbol() => Localeex::Current => GeneralSymbol::Current.symbol(),
            "EN".to_string() => Localeex::English,
            "CH".to_string() => Localeex::Chinese,
            "JP".to_string() => Localeex::Japanese,
            "FR".to_string() => Localeex::French,
            "IT".to_string() => Localeex::Italian,
            "GR".to_string()=> Localeex::German,
            "SP".to_string() => Localeex::Spanish,      
        }
    } */


    pub fn code(&self) -> String {
        match *self {
            Localeex::Current => GeneralSymbol::Current.symbol(),
            Localeex::English => "EN".to_string(),
            Localeex::Chinese => "CH".to_string(),
            Localeex::Japanese => "JP".to_string(),
            Localeex::French => "FR".to_string(),
            Localeex::Italian => "IT".to_string(),
            Localeex::German => "GR".to_string(),
            Localeex::Spanish => "SP".to_string(),
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
        let some_code=code_opt.unwrap_or(GeneralSymbol::Current.symbol());
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

/* /// EnvPropCmd
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvPropCmd {
    pub run_env: RunEnvironment,
    pub locale: Localeex,
    pub key_vals:HashMap<String, EnvPropPack>,
}

impl EnvPropCmd {
    pub fn new() -> EnvPropCmd {
        EnvPropCmd {
            run_env: RunEnvironment::Current,
            locale: Localeex::Current,
            key_vals: HashMap::new(),
        }
    }
}    */ 

/// EnvPropPack
/// one or many environment properties, by running environment & locale
/* #[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvPropVal (String); */

/// EnvPropKey (new type idiom, instead of type alias)
/// comparison case-insensitive, trim-applied 
#[derive(Debug, Clone, Hash,  Eq, Serialize, Deserialize)]
pub struct EnvPropKey(pub String);

impl PartialEq for EnvPropKey {
    fn eq(&self, other: &Self) -> bool {
        // self.0 - takes the first & only one property of the struct
        self.0.to_uppercase().trim() == other.0.to_uppercase().trim()
    }
}

/// type alias. 
type EnvPropVal=String;

/// EnvPropCKey
/// composite/compound key of an environment property
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvPropCKey {
    pub prop_key: EnvPropKey,
    pub run_env: RunEnvironment,
    pub locale: Localeex,
}

impl EnvPropCKey {    
    pub fn new_born(a_key: EnvPropKey, a_run_env: Option<RunEnvironment>, a_locale: Option<Localeex>) -> EnvPropCKey {
        EnvPropCKey {
            prop_key: a_key,
            run_env: if a_run_env.is_none() {RunEnvironment::Current} else {a_run_env.unwrap()},
            locale: if a_locale.is_none() {Localeex::Current} else {a_locale.unwrap()},
        }
    }
}



//#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvPropPack {
    pub key_vals:HashMap<EnvPropCKey, EnvPropVal>,
    pub run_env_strategy:OnDataAvailStrategy,
    pub locale_strategy:OnDataAvailStrategy,
}

impl EnvPropPack {
    // EXpress version of new_born
    // each element of EnvPropCKey and EnvPropVal is laid down and inserted into key_vals at one go
    pub fn new_born_ex(a_prop_key: EnvPropKey, a_run_env: RunEnvironment, a_locale: Localeex, a_prop_val: EnvPropVal, a_re_stgy: Option<OnDataAvailStrategy>, a_loc_stgy: Option<OnDataAvailStrategy>) -> EnvPropPack {
        let epkey1 = EnvPropCKey::new_born(a_prop_key, Some(a_run_env), Some(a_locale));
        let mut epkv1=HashMap::new();
        epkv1.insert(epkey1,a_prop_val);
        EnvPropPack::new_born(epkv1, a_re_stgy, a_loc_stgy)
    }

    pub fn new_born(a_key_vals: HashMap<EnvPropCKey, EnvPropVal>, a_re_stgy: Option<OnDataAvailStrategy>, a_loc_stgy: Option<OnDataAvailStrategy>) -> EnvPropPack {
        EnvPropPack {
            key_vals: a_key_vals,
            run_env_strategy: if a_re_stgy.is_none() {OnDataAvailStrategy::Inherit} else {a_re_stgy.unwrap()},
            locale_strategy: if a_loc_stgy.is_none() {OnDataAvailStrategy::Inherit} else {a_loc_stgy.unwrap()},
        }
    }

    // returns a Property when a matching key is found
    pub fn get_prop(&self, a_key: EnvPropCKey) {
    //pub fn get_val(&self, a_key: EnvPropCKey) -> Option<EnvPropVal> {
        match &self.key_vals.get(&a_key) {
            Some(a_val) => println!("{:?}: {a_val}",a_key.prop_key),
            //Some(a_val) => Some(a_val),
            None => println!("{:?} is not found.",a_key.prop_key)
            //None => None,
        }
    }

    /// get property value with individual elements, 
    /// i.e. not single unit of EnvPropCKey but individual field of the composite key
    /// - a_prop_key: property key to find
    /// - a_run_env: running environment to find; likely the current running environment of the app
    /// - a_locale: locale to find; likely the current locale of the app
    /// output:
    /// - o_prop_val (EnvPropVal) or ErrorOnthefly
    pub fn get_prop_ex(&self, a_prop_key: EnvPropKey, a_run_env: RunEnvironment, a_locale: Localeex, a_parent_re_stgy: Option<OnDataAvailStrategy>, a_parent_loc_stgy: Option<OnDataAvailStrategy>) -> std::result::Result<EnvPropVal,ErrorOnthefly> {
        let mut is_condition_met: bool = false;
        let mut try_cnt: i8 = 0;
        const TRY_CNT_LMT: i8 = 3;

        /// blank, means not found
        let mut o_prop_val = "".to_string();

        /// Loop up to 4 times for 4 retries with different conditions
        /// Try 0: Both RunEnvironment & Locale must match
        /// then next loop if not found ...
        /// Try 1: RunEnvironment default to current, whereas Locale must match
        /// then next loop if not found ...
        /// Try 2: Both RunEnvironment & Locale default to current
        /// then next loop if not found ...
        /// Try 3: RunEnvironment must match, whereas Locale default to current
        ///
        /// * Default to Current is allowed provided that corresponding conditions are met for RunEnv & Locale strategies respectively:
        /// a. strategy is to get default; or
        /// b. strategy is to get inherit and the parent strategy is to get default
        while o_prop_val == "".to_string() && try_cnt <= TRY_CNT_LMT {
            /// check if strategy conditions are met (Try 0 - no strategy check is required)
            let mut is_condition_met = match try_cnt {
                0 => true,
                1 => (self.run_env_strategy==OnDataAvailStrategy::DefaultOnUnavail || (self.run_env_strategy==OnDataAvailStrategy::Inherit && a_parent_re_stgy==Some(OnDataAvailStrategy::DefaultOnUnavail))),
                2 => (self.run_env_strategy==OnDataAvailStrategy::DefaultOnUnavail || (self.run_env_strategy==OnDataAvailStrategy::Inherit && a_parent_re_stgy==Some(OnDataAvailStrategy::DefaultOnUnavail))) && (self.locale_strategy==OnDataAvailStrategy::DefaultOnUnavail || (self.locale_strategy==OnDataAvailStrategy::Inherit && a_parent_loc_stgy==Some(OnDataAvailStrategy::DefaultOnUnavail))),
                3 => (self.locale_strategy==OnDataAvailStrategy::DefaultOnUnavail || (self.locale_strategy==OnDataAvailStrategy::Inherit && a_parent_loc_stgy==Some(OnDataAvailStrategy::DefaultOnUnavail))),
                _ => false,
            };
            if o_prop_val == "".to_string() && is_condition_met {
                let mut a_env_prop_key = match try_cnt {
                    0 => EnvPropCKey::new_born(a_prop_key.clone(), Some(a_run_env.clone()), Some(a_locale.clone())),
                    1 => EnvPropCKey::new_born(a_prop_key.clone(), Some(RunEnvironment::Current), Some(a_locale.clone())),
                    2 => EnvPropCKey::new_born(a_prop_key.clone(), Some(RunEnvironment::Current), Some(Localeex::Current)),
                    3 => EnvPropCKey::new_born(a_prop_key.clone(), Some(a_run_env.clone()), Some(Localeex::Current)),
                    _ => EnvPropCKey::new_born(a_prop_key.clone(), None, None),
                };
                ///
                /// get property value, given the EnvPropCKey - composite of prop key, running environment & locale
                /// exceptional if try_cnt exceeds limit
                if try_cnt <= TRY_CNT_LMT {
                    match &self.key_vals.get(&a_env_prop_key) {
                        Some(a_val) => o_prop_val = a_val.clone().to_string(),
                        None => o_prop_val = "".to_string(),
                    }
                }
            }
            try_cnt = try_cnt + 1;
        }

/*         let mut a_env_prop_key = EnvPropCKey::new_born(a_prop_key.clone(), Some(a_run_env.clone()), Some(a_locale.clone()));

        let mut o_prop_val = "".to_string();

        match &self.key_vals.get(&a_env_prop_key) {
            Some(a_val) => o_prop_val = a_val.clone().to_string(),
            None => o_prop_val = "".to_string(),
        }

        /// Try 1:
        /// if value is not found, try getting from current runenvironment 
        /// 1st try with the given locale
        /// provided that:
        /// a. strategy is to get default; or
        /// b. strategy is to get inherit and the parent strategy is to get default
        if o_prop_val == "".to_string() {
            if (self.run_env_strategy==OnDataAvailStrategy::DefaultOnUnavail || (self.run_env_strategy==OnDataAvailStrategy::Inherit && a_parent_re_stgy==Some(OnDataAvailStrategy::DefaultOnUnavail))) {
                let mut a_env_prop_key = EnvPropCKey::new_born(a_prop_key.clone(), Some(RunEnvironment::Current), Some(a_locale));
                match &self.key_vals.get(&a_env_prop_key) {
                    Some(a_val) => o_prop_val = a_val.clone().to_string(),
                    None => o_prop_val = "".to_string(),
                }

                /// Try 2:
                /// if still not found, try getting from current runenvironment 
                /// next try with current locale as well
                /// provided that both the runenv both locale strategies get to default
                if o_prop_val == "".to_string() {
                    if (self.locale_strategy==OnDataAvailStrategy::DefaultOnUnavail || (self.locale_strategy==OnDataAvailStrategy::Inherit && a_parent_loc_stgy==Some(OnDataAvailStrategy::DefaultOnUnavail))) {
                        let mut a_env_prop_key = EnvPropCKey::new_born(a_prop_key.clone(), Some(RunEnvironment::Current), Some(Localeex::Current));
                        match &self.key_vals.get(&a_env_prop_key) {
                            Some(a_val) => o_prop_val = a_val.clone().to_string(),
                            None => o_prop_val = "".to_string(),
                        }
                    }
                }
             }
        }


        /// Try 3:
        /// if still not found, try getting from current locale, but with the given runenv 
        /// provided that the locale strategy get to default
        if o_prop_val == "".to_string() {
            if (self.locale_strategy==OnDataAvailStrategy::DefaultOnUnavail || (self.locale_strategy==OnDataAvailStrategy::Inherit && a_parent_loc_stgy==Some(OnDataAvailStrategy::DefaultOnUnavail))) {
                let mut a_env_prop_key = EnvPropCKey::new_born(a_prop_key.clone(), Some(a_run_env), Some(Localeex::Current));
                match &self.key_vals.get(&a_env_prop_key) {
                    Some(a_val) => o_prop_val = a_val.clone().to_string(),
                    None => o_prop_val = "".to_string(),
                }
            }
        } */

        if o_prop_val == "".to_string() {
            Err(ErrorOnthefly::RecordNotFound)
        } else {
            Ok(o_prop_val)
        }   

    }

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
    pub env_props:HashMap<EnvPropCKey, EnvProp>,
}

impl EnvPropSet {
    /// vanilla new
    /// only running environment is mandatory, whereas the remaining properties are default
    pub fn new(a_run_env: RunEnvironment) -> EnvPropSet {
        EnvPropSet {
            run_env: a_run_env,
            default_locale: Localeex::Current,
            run_env_strategy: OnDataAvailStrategy::DefaultOnUnavail,
            locale_strategy: OnDataAvailStrategy::DefaultOnUnavail,
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
            run_env_strategy:OnDataAvailStrategy::Inherit,
            locale_strategy:OnDataAvailStrategy::Inherit,
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
            env:RunEnvironment::Current,
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

#[cfg(test)]
mod tests {
    /// Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use rand::distributions::{Alphanumeric, DistString};
    use rand::Rng;
    //use iif::iif;

    /// test comparison of EnvPropCKey - composite key
    /// expected: case insensitive and trim applied
    #[test]
    fn test_comp_env_prop_ckey() {  
        let epk1: EnvPropKey = EnvPropKey("  en  ".to_string());
        let epck1 = EnvPropCKey::new_born(epk1, Some(RunEnvironment::UAT), Some(Localeex::Current));

        let epk2: EnvPropKey = EnvPropKey("EN".to_string());
        let epck2 = EnvPropCKey::new_born(epk2, Some(RunEnvironment::UAT), Some(Localeex::Current));
        
        assert_eq!(epck1, epck2);    
    }

/*     /// setup and return an env property cmd with testing data
    fn test_setup_env_prop_cmd(a_epp_key: &str) -> EnvPropCmd {
        let t_epcmd = EnvPropCmd::new();
        let t_epp_key1 = a_epp_key.to_string();
        let t_eppack1 = test_setup_env_prop_pack(&t_epp_key1);
        t_epcmd.key_vals.insert(t_epp_key1, t_eppack1);
  
        let t_epp_key2 = "Enter".to_string();
        let t_eppack2 = test_setup_env_prop_pack(&t_epp_key2);
        t_epcmd.key_vals.insert(t_epp_key2, t_eppack2);

        t_epcmd
    } */

    /// setup and return an env property pack with testing data
    fn test_setup_env_prop_pack(a_epp_key: &EnvPropKey) -> EnvPropPack {
        let t_epp_key = a_epp_key;
        let mut t_eppack = EnvPropPack::new_born_ex(t_epp_key.clone(), RunEnvironment::DEV, Localeex::Chinese, "退出".to_string(), Some(OnDataAvailStrategy::DefaultOnUnavail), Some(OnDataAvailStrategy::Inherit));
        t_eppack.key_vals.insert(EnvPropCKey::new_born(t_epp_key.clone(), Some(RunEnvironment::Current), Some(Localeex::Current)),"Quit".to_string());
        t_eppack.key_vals.insert(EnvPropCKey::new_born(t_epp_key.clone(), Some(RunEnvironment::PROD("DR server1".to_string())), Some(Localeex::Japanese)),"やめる".to_string());

        t_eppack 
    }

    /// ok for exact match of key, run env & locale
    #[test]
    fn test_get_env_prop_exact_ok() {  
        /// property key for testing is randomly generated value of a random len between 1 to 100
        let t_epp_key = EnvPropKey(Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(1..100)));
        let t_eppack_val = test_setup_env_prop_pack(&t_epp_key).get_prop_ex(t_epp_key.clone(),RunEnvironment::PROD("DR server1".to_string()), Localeex::Japanese, Some(OnDataAvailStrategy::ErrOnUnavil), Some(OnDataAvailStrategy::ErrOnUnavil));
        assert_eq!(t_eppack_val.ok().unwrap(), "やめる".to_string());    
    }

    /// error for exact match of key, run env & locale
    #[test]
    fn test_get_env_prop_exact_err() {  
        let t_epp_key = EnvPropKey("quit".to_string());
        let t_eppack_val = test_setup_env_prop_pack(&t_epp_key).get_prop_ex(t_epp_key.clone(),RunEnvironment::DEV, Localeex::Japanese, Some(OnDataAvailStrategy::ErrOnUnavil), Some(OnDataAvailStrategy::ErrOnUnavil));
        assert_eq!(t_eppack_val.err().unwrap(), ErrorOnthefly::RecordNotFound);    
    }

    /// ok inherit parent/up-one-level locale strategy that defaults to get the current locale when given locale is not found
    #[test]
    fn test_get_env_prop_inherit_cur_ok() {  
        let t_epp_key = EnvPropKey("quit".to_string());
        let t_eppack_val = test_setup_env_prop_pack(&t_epp_key).get_prop_ex(t_epp_key.clone(),RunEnvironment::DEV, Localeex::English, Some(OnDataAvailStrategy::DefaultOnUnavail), Some(OnDataAvailStrategy::DefaultOnUnavail));
        assert_eq!(t_eppack_val.ok().unwrap(), "Quit".to_string());    
    }

    /// error inherit parent/up-one-level locale strategy that returns error when given locale is not found
    #[test]
    fn test_get_env_prop_inherit_cur_err() {  
        let t_epp_key = EnvPropKey("quit".to_string());
        let t_eppack_val = test_setup_env_prop_pack(&t_epp_key).get_prop_ex(t_epp_key.clone(),RunEnvironment::DEV, Localeex::English, Some(OnDataAvailStrategy::DefaultOnUnavail), Some(OnDataAvailStrategy::ErrOnUnavil));
        assert_eq!(t_eppack_val.err().unwrap(), ErrorOnthefly::RecordNotFound);    
    }
}
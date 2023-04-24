use std::collections::HashMap;

// ========================================================================
// HashMap example
// Prop - application property/environment variable - key, value construct
// Props - locale (EN), Staging Environment (DEV/PROD)
// and properties (a HashMap of Prop)
// ========================================================================

#[derive(Debug)]
pub enum StagingEnvironment {
    DEV,
    UAT,
    SIT,
    PROD{label:String}, // label is used when there is a need to refine meaning of PRODuction env. (e.g. PROD, DR, etc.)
}

#[derive(Debug,Clone)]
pub struct Locale {
    pub code: String,
    pub description: String,
}

impl Locale {
    // description is optional
    pub fn new(code: String, descr_opt: Option<String>) -> Locale {
        let mut some_descr:String;
/*         if let Some(descr)=some_descr {
            some_descr=descr;
        } */
        match descr_opt {
            Some(descr)=>some_descr=descr,
            None => some_descr="".to_string(),
        }
        Locale { code: code, description: some_descr }
    }
}

#[derive(Debug,Clone)]
pub struct Prop {
    pub key: String,
    pub val: String,
}

impl Prop {
    pub fn new(key: String, val: String) -> Prop {
        Prop { key: key, val: val }
    }
}

// HashMap of Prop (application properties)
// 1st element is the (K)ey, 2nd element is the (V)alue
// HashMap implementation prevents duplicate key
#[derive(Debug,Clone)]
pub struct Props {
    /*pub locale:Locale,
    pub env:StagingEnvironment,*/
    pub props:HashMap<String, String>,
}

impl Props {
    pub fn new() -> Props {
        Props {
            // properties collection defined as HashMap, instead of struct Prop
            // to take advantage of unique key mechanism
            props: HashMap::new(),
        }
    }

    // a Property joins as member
    pub fn join(
        &mut self, // must be mutable
        prop: Prop,
    ) {
        self.join_with_raw_data(prop.key, prop.val);
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

pub struct PropSet  {
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
  
  }
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

pub fn get_propss()->HashMap<String,String>{
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
}
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
use serde_json::{Number, Value};
use serde_derive::{Deserialize, Serialize};
use serde_json::Map;

use crate::core::env::{Prop};

// https://blog.devgenius.io/reading-and-writing-a-json-file-in-rust-2731da8d6ad0
#[derive(Deserialize, Serialize, Debug)]
struct Food {
    id: u32,
    name: String,
    //missy_likeness: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Schedule {
    date: i64,
    quantity: f64,
    food: u32,
    missy_grumpiness: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct MissyFoodSchedule {
    food: Vec<Food>,
    missy_food_schedule: Vec<Schedule>,
}

pub fn test_json_stat_read(input_path: String) -> Result<(), std::io::Error> {
/*     let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap(); */
    let mut missy_secrets = {
        let missy_secrets = std::fs::read_to_string(&input_path)?;

        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<MissyFoodSchedule>(&missy_secrets).unwrap()
    };

    // Double the quantity for each carbona in 'missy_food_schedule'
    for index in 0..missy_secrets.missy_food_schedule.len() {
        missy_secrets.missy_food_schedule[index].quantity *= 2.;
        println!("missy food schedule: {}",missy_secrets.missy_food_schedule[index].quantity.to_string());
    }

/*     // Save the JSON structure into the output file
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&missy_secrets).unwrap(),
    )?; */

    Ok(())
}

// https://blog.devgenius.io/reading-and-writing-a-json-file-in-rust-2731da8d6ad0
pub fn test_json_dyn_read() {
    // Get the filenames from the command line.
/*     let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap(); */
    let input_path = "./assets/missy.json";

    let mut missy_diet = {
        // Load the first file into a string.
        let text = std::fs::read_to_string(&input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    // Get the number of carbonas in the object 'missy_food_schedule'
    let nb_carbonas = missy_diet["missy_food_schedule"].as_array().unwrap().len();

    for index in 0..nb_carbonas{
        if let Value::Number(n) = &missy_diet["missy_food_schedule"][index]["quantity"] {
            // Double the quantity for each carbona in 'missy_food_schedule'
            missy_diet["missy_food_schedule"][index]["quantity"] =
                Value::Number(Number::from_f64(n.as_f64().unwrap() * 2.).unwrap());
                println!("missy food schedule: {}",missy_diet["missy_food_schedule"][index]["quantity"].to_string());
        }
    }

/*     // Save the JSON structure into the other file.
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&missy_diet).unwrap(),
    )
    .unwrap(); */
}

pub fn json_to_env_prop_pack(a_value: &Value) -> Vec<String> {
    let mut o_vec = Vec::new();

    /// an array of objects
    if a_value.is_array() {
        let item_cnt = a_value.as_array().unwrap().len();

        /// iterate the array
        for index in 0..item_cnt{
            let mut a_obj = a_value[index].as_object().unwrap();
            for (key, value) in a_obj.iter() {
                //println!("sjikan, {:?}", value);
                let mut o_result = match *value {
                    Value::String(ref v) => vec![v.to_string()],
                    Value::Object(ref map)=> json_to_env_prop_pack(&Value::Object(map.clone())),
                    Value::Array(ref vec)=> json_to_env_prop_pack(&Value::Array(vec.clone())),
                    _ => vec![],
                };
                o_vec.push(o_result[0].clone());
            }    
        }

    } else {
        let mut a_obj = a_value.as_object().unwrap();
        for (key, value) in a_obj.iter() {
            //println!("sjikan, {:?}", value);
            let mut o_result = match *value {
                Value::String(ref v) => vec![v.to_string()],
                Value::Object(ref map)=> json_to_env_prop_pack(&Value::Object(map.clone())),
                Value::Array(ref vec)=> json_to_env_prop_pack(&Value::Array(vec.clone())),
                _ => vec![],
            };
            o_vec.push(o_result[0].clone());
        }    
    }

    let o_item_cnt = o_vec.len();
    for o_idx in 0..o_item_cnt{
        println!("o_vector: {}{:?}", o_idx.to_string(), o_vec[o_idx]);
    }
    o_vec
}

pub fn json_to_env_prop_packa(a_value: &Value) -> Vec<String> {
    let mut o_vec = Vec::new();

    /// an array of objects
    if a_value.is_array() {
        let item_cnt = a_value.as_array().unwrap().len();

        /// iterate the array
        for index in 0..item_cnt{
            let mut a_obj = a_value[index].as_object().unwrap();
            for (key, value) in a_obj.iter() {
                //println!("sjikan, {:?}", value);
                let mut o_result = match *value {
                    Value::String(ref v) => vec![v.to_string()],
                    Value::Object(ref map)=> json_to_env_prop_packa(&Value::Object(map.clone())),
                    Value::Array(ref vec)=> json_to_env_prop_packa(&Value::Array(vec.clone())),
                    _ => vec![],
                };
                o_vec.push(o_result[0].clone());
            }    
        }

    } else {
        let mut a_obj = a_value.as_object().unwrap();
        for (key, value) in a_obj.iter() {
            //println!("sjikan, {:?}", value);
            let mut o_result = match *value {
                Value::String(ref v) => vec![v.to_string()],
                Value::Object(ref map)=> json_to_env_prop_packa(&Value::Object(map.clone())),
                Value::Array(ref vec)=> json_to_env_prop_packa(&Value::Array(vec.clone())),
                _ => vec![],
            };
            o_vec.push(o_result[0].clone());
        }    
    }

    let o_item_cnt = o_vec.len();
    for o_idx in 0..o_item_cnt{
        println!("o_vector: {}{:?}", o_idx.to_string(), o_vec[o_idx]);
    }
    o_vec
}

pub fn test_read_json_prop_file() {  
    let input_path = "./assets/app_properties.json";

    let mut app_props = {
        // Load the first file into a string.
        let app_props = std::fs::read_to_string(&input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&app_props).unwrap()
    };

    println!("array of objs: {:?}", json_to_env_prop_pack(&app_props["env_props"]["properties"]));

    //println!("app_properties.json: {}",app_props["env_props"]["locale"].to_string());
    // Get the number of items in the object 'env_props'
    let item_cnt = app_props["env_props"]["properties"].as_array().unwrap().len();
    //let item_cnt = app_props["env_props"]["properties"][0].as_object().unwrap().len();

    for index in 0..item_cnt{
 /*        let mut obj0 = app_props["env_props"]["properties"][index].as_object().unwrap();
        for (key0, value0) in obj0.iter() {
            let mut a_map=Map::new();
            //let a_map0=value0.clone();
            //a_map.insert(key0.to_string(), a_map0);
            a_map.insert(key0.to_string(), value0.clone());
            //println!("j2e: {}", json_to_env_prop_pack(&a_map));
        } */

        let mut obj = app_props["env_props"]["properties"][index].as_object().unwrap();

        for (key, value) in obj.iter() {
            println!("obj {}: {}", key.clone(), match *value {
                //Value::U64(v) => format!("{} (u64)", v),
                Value::String(ref v) => format!("{} (string)", v),
                _ => format!("other")
            });

            //let mut obj1 = app_props["env_props"]["properties"][index][key.to_string()].as_object().unwrap();
            let mut obj1 = app_props["env_props"]["properties"][index][key][0].as_object().unwrap();
            for (key1, value1) in obj1.iter() {
                println!("obj1 {}: {}", key1.clone(), match *value1 {
                    //Value::U64(v) => format!("{} (u64)", v),
                    Value::String(ref v) => format!("{} (string)", v),
                    _ => format!("other")
                });
            }
        }
        //println!("app_properties.json: {}",app_props["env_props"]["properties"][0].as_object().unwrap());
       // println!("app_properties.json: {}",app_props["env_props"]["properties"][index]["locale"].to_string());
    }
}


#[cfg(test)]
mod tests {
    /// Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use rand::distributions::{Alphanumeric, DistString};
    use rand::Rng;


}
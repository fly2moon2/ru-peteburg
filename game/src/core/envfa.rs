use serde_json::{Number, Value};
use serde_derive::{Deserialize, Serialize};

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

    // Double the quantity for each element in 'missy_food_schedule'
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

    // Get the number of elements in the object 'missy_food_schedule'
    let nb_elements = missy_diet["missy_food_schedule"].as_array().unwrap().len();

    for index in 0..nb_elements{
        if let Value::Number(n) = &missy_diet["missy_food_schedule"][index]["quantity"] {
            // Double the quantity for each element in 'missy_food_schedule'
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
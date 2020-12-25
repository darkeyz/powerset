use std::env;
use std::process;

use powerset::PowerSet;
use powerset::InputSet;

fn main() {
    // Parse input
    let input = InputSet::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Invalid input: {}", err);
        process::exit(1);
    });

    let parsed_set: Vec<&str> = input.set.split(",").collect();

    // Handle empty inputs
    let invalid_string = parsed_set.iter().any(|val| val.is_empty() );
    if invalid_string {
        eprintln!("Invalid input (Empty string)"); 
        process::exit(1);
    }

    let mut powerset = PowerSet { data: parsed_set, powerset: vec!(String::from("")) };
    powerset.generate_powerset();
    powerset.print();
}


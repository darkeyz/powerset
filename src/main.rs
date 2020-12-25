use std::env;
use std::process;

use powerset::InputSet;
use powerset::compare_len;

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

    let mut powerset = vec!();
    
    // Empty set
    powerset.push(String::from(""));

    for set_el in parsed_set {
        for j in 0..powerset.len() {
            let new_set = if powerset[j].is_empty()
                { format!("{}", set_el) } 
                else 
                { format!("{},{}", &powerset[j], set_el) };
            powerset.push(new_set);
        }
    }
    
    // Sort by string length (set size)
    powerset.sort_by(|a,b| compare_len(&a, &b));
    println!("{:?}", powerset);
}


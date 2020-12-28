use std::env;
use std::process;

fn main() {
    // Parse input
    let input = powerset::InputSet::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    powerset::into_powerset(input.set).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
}

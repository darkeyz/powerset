use std::env;
use std::error::Error;
use std::fs;

/// Contains input types (raw|file)
#[derive(Debug)]
pub enum InputType {
    Raw,
    File,
}

/// Contains input and parsing option
// #[derive(Debug)]
pub struct InputSet {
    pub set: String,
    pub input_type: InputType,
}

impl InputSet {
    /// Parses and validate the string input and options before creating the instance
    pub fn new(mut args: env::Args) -> Result<InputSet, &'static str> {
        args.next();

        let input_type = match args.next() {
            Some(arg) => match &arg[..] {
                "-i" => InputType::Raw,
                "-f" => InputType::File,
                _ => return Err("Invalid option! (Use `-i <raw_input>` or `-f <file_path>`)"),
            },
            _ => return Err("Empty option! (Use `-i <raw_input>` or `-f <file_path>`)"),
        };

        let set_arg = match args.next() {
            Some(arg) => arg,
            _ => return Err("Empty input! (Use `-i <raw_input>` or `-f <file_path>`)"),
        };

        let set = match input_type {
            InputType::Raw => set_arg,
            InputType::File => match readfile(set_arg) {
                Ok(content) => content,
                _ => return Err("Couldn't read file!"),
            },
        };

        let parsed_set: Vec<&str> = set.split(",").collect();

        let invalid_string =
            parsed_set
                .iter()
                .enumerate()
                .any(|(i, val)| if i > 0 { val.is_empty() } else { false }); // validate input
        if invalid_string {
            return Err("Invalid input/sets found!");
        }

        Ok(InputSet { set, input_type })
    }
}

/// Prints, and returns the powerset from a string input 
pub fn into_powerset(set: String) -> Result<String, &'static str> {
    // println!("{:?}", set);
    let parsed_set: Vec<&str> = set.split(",").collect();

    let mut powerset = vec![String::from("")];

    let empty_set = parsed_set.iter().all(|val| val.trim().is_empty()); // empty set/file is accepted

    // Handle empty set's'powerset
    if !(empty_set) {
        for set_el in parsed_set.iter() {
            for j in 0..powerset.len() {
                // Create new set based on current one adding the new item
                let new_set = if powerset[j].is_empty() {
                    format!("{}", set_el)
                } else {
                    format!("{},{}", &powerset[j], set_el)
                };
                // Concat new set to existing powerset
                powerset.push(
                    new_set
                        .trim_matches(char::is_whitespace) // trim end/start whitspaces
                        .replace(char::is_whitespace, ""), // clean all other whitspaces
                );
            }
        }
    }
    // println!("{:?}", powerset);
    // Sort by string length (set size)
    powerset.sort_by(|a, b| compare_len(&a, &b)); // Sort the powerset
    let output = powerset.join("\n");
    print!("{}", output);
    Ok(output)
}

fn readfile(file_path: String) -> Result<String, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    Ok(file_content
        .trim_matches(char::is_whitespace)
        .replace(char::is_whitespace, ""))
}

fn compare_len(a: &str, b: &str) -> std::cmp::Ordering {
    match a.len() < b.len() {
        true => std::cmp::Ordering::Less,
        false => std::cmp::Ordering::Greater,
    }
}

use std::env;
// use std::error::Error;

// #[derive(Debug)]
pub struct InputSet {
    pub set: String,
}

impl InputSet {
    pub fn new(mut args: env::Args) -> Result<InputSet, &'static str> {
        args.next();

        let set = match args.next() {
            Some(arg) => arg,
            None => return Err("No string given!"),
        };

        Ok(InputSet { set })
    }
}

pub fn compare_len(a: &str, b: &str) -> std::cmp::Ordering {
    match a.len() < b.len() {
        true => std::cmp::Ordering::Less,
        false => std::cmp::Ordering::Greater,
    }
}

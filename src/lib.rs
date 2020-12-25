use std::env;

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

#[derive(Debug)]
pub struct PowerSet<'a> {
    pub data: Vec<&'a str>,
    pub powerset: Vec<String>,
}

impl<'a> PowerSet<'a> {
    pub fn new(data: Vec<&'a str>) -> PowerSet<'a> {
        PowerSet { data, powerset: vec!(String::from("")) }
    }

    pub fn generate_powerset(&mut self) {
        for set_el in &self.data {
            for j in 0..self.powerset.len() {
                let new_set = if self.powerset[j].is_empty()
                    { format!("{}", set_el) } 
                    else 
                    { format!("{},{}", &self.powerset[j], set_el) };
                self.powerset.push(new_set);
            }
        }
        // Sort by string length (set size)
        self.powerset.sort_by(|a,b| PowerSet::compare_len(&a, &b));
    }

    pub fn print(&self) {
        let output = self.powerset.join("\n");
        println!("{}", output);
    }

    pub fn format(&self) -> String {
        let output = self.powerset.join("\n");
        output
    }

    fn compare_len(a: &str, b: &str) -> std::cmp::Ordering {
        match a.len() < b.len() {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Greater,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_input_test () {
        let input = vec!("123", "456", "789");
        let mut powerset = PowerSet { data: input, powerset: vec!(String::from("")) };
        powerset.generate_powerset();
        assert_eq!("
123
456
789
123,456
123,789
456,789
123,456,789", powerset.format());
    }
}

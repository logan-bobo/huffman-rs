use std::{char, collections::HashMap, error::Error, fs, usize};

pub struct Config {
    pub file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("Incorrect arguments supplied");
        }

        let file = args[1].clone();

        Ok(Config { file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    let char_table = generate_char_table(contents);

    println!("{char_table:#?}");

    Ok(())
}

fn generate_char_table(contents: String) -> HashMap<char, usize> {
    let mut char_map: HashMap<char, usize> = HashMap::new();

    for char in contents.chars() {
        let value = char_map.entry(char).or_insert(0);
        *value += 1;
    }

    char_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_char_table() {
        let content: String = "hello".to_string();
        let table: HashMap<char, usize> = HashMap::from([('h', 1), ('e', 1), ('l', 2), ('o', 1)]);

        assert_eq!(table, generate_char_table(content))
    }
}

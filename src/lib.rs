use std::{char, collections::HashMap, error::Error, fs, usize};

pub struct Config {
    pub file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
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
    contents.chars().fold(HashMap::new(), |mut acc, char| {
        *acc.entry(char).or_insert(0) += 1;
        acc
    })
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

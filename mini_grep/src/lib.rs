use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn create(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Not Enough Arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let conent =
        fs::read_to_string(config.file_path).expect("Should be able to read path to the file");

    println!("Contents of the file:\n{conent}");
    Ok(())
}

mod private {
        pub const PREFIX: &'static str = "'";
}

use std::fs::File;
use std::io::prelude::*;
use std::io::{Error};


pub struct Config {
        token: String,
        prefix: &'static str,
}
impl Config {
        pub fn new() -> Config {
                let token: String = match read_file_token() {
                        Ok(s) => s,
                        Err(_) => {
                                println!("Error reading the token file");
                                String::from("")
                        }
                };
                return Config {
                        token: token,
                        prefix: private::PREFIX,
                };
        }
        pub fn token(&self) -> &String {
                return &self.token;
        }
        pub fn prefix(&self) -> &'static str {
                return &self.prefix;
        }
}


fn read_file_token() -> Result<String, Error> {
        let mut file: File = File::open("./_resources/token.dat")?;
        let mut content: String = String::new();
        file.read_to_string(&mut content)?;

        Ok(content)
}
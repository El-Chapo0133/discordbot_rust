mod private {
        pub const TOKEN: &'static str = "";
        pub const PREFIX: &'static str = "!";
}


pub struct Config {
        token: &'static str,
        prefix: &'static str,
}
impl Config {
        pub fn new() -> Config {
                return Config {
                        token: private::TOKEN,
                        prefix: private::PREFIX,
                }
        }
        pub fn token(&self) -> &'static str {
                return &self.token;
        }
        pub fn prefix(&self) -> &'static str {
                return &self.prefix;
        }
}
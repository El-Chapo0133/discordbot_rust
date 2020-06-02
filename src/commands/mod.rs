


pub struct Commands {
        all: [String; 4],
}

impl Commands {
        pub fn new() -> Commands {
                return Commands {
                        all: [String::from("mine"),String::from("upgrade"),String::from("engage"),String::from("stats")],
                }
        }
        pub fn all(&self) -> &[String; 4] {
                return &self.all;
        }
        pub fn is_in(&self, input: &str) -> bool {
                true
        }
}

struct DiscordHelp {}
impl DiscordHelp {
        fn new_embed(title: String) {
                
        }
}
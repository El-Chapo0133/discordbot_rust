



pub struct Commands {
        all: [String; 4],
}

impl Commands {
        pub fn new() -> Commands {
                return Commands {
                        all: ["mine","upgrade","engage","stats"],
                }
        }
        pub fn all(&self) -> [String; 4] {
                return self.all;
        }
}

struct DiscordHelp {}
impl DiscordHelp {
        new_embed() {

        }
}
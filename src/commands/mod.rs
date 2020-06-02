


pub struct ListCommands {
        all: [String; 6],
}

impl ListCommands {
        pub fn new() -> ListCommands {
                return ListCommands {
                        all: [String::from("'init"),String::from("'mine"),String::from("'upgrade"),String::from("'engage"),String::from("'stats"),String::from("'help")],
                }
        }
        pub fn all(&self) -> &[String; 6] {
                return &self.all;
        }
        pub fn is_in(&self, input: &str) -> bool {
                for command in self.all() {
                        if command == input {
                                return true;
                        }
                }
                return false;
        }
}
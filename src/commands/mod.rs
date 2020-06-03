


pub struct ListCommands {
        all: [String; 8],
}

impl ListCommands {
        pub fn new() -> ListCommands {
                return ListCommands {
                        all: [String::from("'collect"),String::from("'show"),String::from("'init"),String::from("'mine"),String::from("'upgrade"),String::from("'engage"),String::from("'stats"),String::from("'help")],
                }
        }
        pub fn all(&self) -> &[String; 8] {
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


#[cfg(test)]
mod tests {

        use super::*;

        #[test]
        fn should_be_true() {
                let list_commands: ListCommands = ListCommands::new();
                assert_eq!(list_commands.is_in("'init"), true);
                assert_eq!(list_commands.is_in("'stats"), true);
                assert_eq!(list_commands.is_in("'mine"), true);
                assert_eq!(list_commands.is_in("'upgrade"), true);
        }

        #[test]
        fn should_be_false() {
                let list_commands: ListCommands = ListCommands::new();
                assert_eq!(list_commands.is_in("'osuef"), false);
                assert_eq!(list_commands.is_in("'stat"), false);
                assert_eq!(list_commands.is_in("'Mine"), false);
                assert_eq!(list_commands.is_in("'uptredf"), false);
        }
}
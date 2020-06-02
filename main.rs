

mod src;



use serenity::{
        prelude::*,
        model::prelude::*,
        Client
};
use serde::{Serialize,Deserialize};
use serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Write,Error};

#[derive(Debug,Serialize,Deserialize)]
struct Users {
        users: Vec<User>,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct User {
        name: String,
        golds: f64,
}

impl Users {
        fn find(&self, input: String) -> User {
                for user in self.users.iter() {
                        if user.name == input {
                                return user.clone();
                        }
                }
                return User { name: String::from(""), golds: 0.0 };
        }
}

struct Handler {
        prefix: &'static str,
}
impl EventHandler for Handler {
        fn message(&self, context: Context, message: Message) {
                if start_with_prefix(&message.content, &self.prefix) {
                        lead_command(context, message);
                }
        }
}

fn lead_command(context: Context, message: Message) {
        let list_commands: src::commands::ListCommands = src::commands::ListCommands::new();
        let command: Vec<&str> = message.content.split_whitespace().collect();
        if list_commands.is_in(&command[0]) {
                // TODO : 
                //  lead all bot functions
                if command[0] == "'help" {
                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                message.embed(|embed| {
                                        embed.title("EarthBreaker HELP");
                                        embed.description("description");
                                        embed.field("command 'mine'", "value", false);

                                        return embed;
                                });

                                return message;
                        }) {
                                println!("Error sending message: {}", why);
                        }
                } else if command[0] == "'mine" {
                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                message.embed(|embed| {
                                        embed.title("Mined !");
                                        embed.description("You've just mined `x` golds\nplease wait `x` minutes before mining again");

                                        return embed;
                                });

                                return message;
                        }) {
                                println!("Error sending message: {}", why);
                        }
                } else if command[0] == "'stats" {
                        let content: String = match read_file("./_resources/users.json") {
                                Ok(s) => s,
                                Err(why) => {
                                        println!("Error reading file: {}", why);
                                        String::from("failed")
                                }
                        };
                        let users: Users = serialize(content);
                        let stat: User = users.find(message.author.name.to_string());
                        
                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                message.embed(|embed| {
                                        embed.title(format!("{} stats", stat.name));
                                        embed.field("Golds", stat.golds.to_string(), false);
                                        return embed;
                                });

                                return message;
                        }) {
                                println!("Error sending message: {}", why);
                        }
                } else if command[0] == "'init" {
                        let user: String = message.author.name.to_string();
                        let content: String = match read_file("./_resources/users.json") {
                                Ok(s) => s,
                                Err(why) => {
                                        println!("Error reading file: {}", why);
                                        String::from("failed")
                                }
                        };
                        let mut users: Users = serialize(content);
                        users.users.push(User {name: user, golds: 0.0});


                        let json: String = deserialize(&users);
                        match write_file(json, "./_resources/users.json") {
                                Ok(()) => (),
                                Err(why) => {
                                        println!("Failed write file: {}", why);
                                        std::process::exit(1)
                                }
                        }
                }
        }
}

fn start_with_prefix(message: &String, prefix: &str) -> bool {
        match message.get(0..prefix.len()) {
                Some(s) => return s == prefix,
                None => return false,
        }
}

fn deserialize(input: &Users) -> String {
        return match serde_json::to_string(input) {
                Ok(json) => json,
                Err(_) => {
                        println!("Could not deserialize");
                        std::process::exit(1)
                }
        };
}

fn serialize(input: String) -> Users {
        match serde_json::from_str(&input) {
                Ok(foo) => return foo,
                Err(why) => {
                        println!("Could not serialize: {}", why);
                        std::process::exit(1)
                }
        }
}

fn write_file(input: String, path: &str) -> Result<(), Error> {
        let mut file: File = File::create(path)?;
        write!(file, "{}", input)?;

        Ok(())
}

fn read_file(path: &str) -> Result<String, Error> {
        let mut file: File = File::open(path)?;
        let mut content: String = String::new();
        file.read_to_string(&mut content)?;

        Ok(content)
}


fn main() {
        let config: src::config::Config = src::config::Config::new();
        
        let handler: Handler = Handler {prefix: config.prefix()};
        let mut client = Client::new(config.token(), handler).expect("Could not creat the client");
        if let Err(why) = client.start() {
                println!("Error on starting the client: {}", why);
                std::process::exit(1);
        }
}











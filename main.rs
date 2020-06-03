

mod src;

mod data {
        pub const TIME_SINCE_MINE: i64 = 10; //60 * 20; // 20min
}

extern crate time;

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
        mine_ref: f64,
        last_mine: i64,
        mine_upgrades: f64,
        miners: i64,
}

impl Users {
        fn find(&self, input: &String) -> Option<User> {
                for user in self.users.iter() {
                        if &user.name == input {
                                return Some(user.clone());
                        }
                }
                return None;
        }
        fn add_golds(&mut self, author: &String) -> Result<(), String> {
                for index in 0..self.users.len() {
                        if &self.users[index].name == author {
                                self.users[index].golds += self.users[index].mine_ref;
                                return Ok(());
                        }
                }
                return Err(format!("Could not find the user {}", author));
        }
        fn update_last_mine(&mut self, author: &String) -> Result<(), String> {
                for index in 0..self.users.len() {
                        if &self.users[index].name == author {
                                self.users[index].last_mine = time::get_time().sec;
                                return Ok(());
                        }
                }
                return Err(format!("Could not find the user {}", author));
        }
        fn upgrade_and_update_mine_ref(&mut self, author: &String) -> Result<(), String> {
                for index in 0..self.users.len() {
                        if &self.users[index].name == author {
                                if self.users[index].golds - src::exponant::Exponant::get(&self.users[index].mine_upgrades, 25.0) < 0.0 {
                                        return Err(String::from("You do not have enough money to buy this"));
                                }
                                self.users[index].mine_upgrades += 1.0;
                                self.users[index].mine_ref += src::logarithm::Logarithm::get(&self.users[index].mine_upgrades);
                                return Ok(());
                        }
                }
                return Err(format!("Could not find user {}, call the bot's admin", author));
        }
        fn get_last_mine(&self, author: &String) -> Option<i64> {
                for user in self.users.iter() {
                        if &user.name == author {
                                return Some(user.last_mine);
                        }
                }
                return None;
        }
        fn buy(&mut self, author: &String, divisor: f64) -> Result<String, String> {
                for index in 0..self.users.len() {
                        if &self.users[index].name == author {
                                self.users[index].golds -= src::exponant::Exponant::get(&self.users[index].mine_upgrades, divisor);
                                return Ok(String::from(&self.users[index].mine_upgrades.to_string()));
                        }
                }
                return Err(format!("Could not find user {}", author));
        }
        fn engage_miner(author: &String) -> Result<(), String> {
                Ok(())
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
                if command[0] == "'help" {
                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                message.embed(|embed| {
                                        embed.title("EarthBreaker HELP");
                                        embed.description("description");
                                        embed.field("command 'mine'", "value", false);
                                        embed.field("command 'stats'", "value", false);
                                        embed.field("command 'init'", "value", false);
                                        embed.field("command 'upgrade'", "value", false);

                                        return embed;
                                });

                                return message;
                        }) {
                                println!("Error sending message: {}", why);
                        }
                } else if command[0] == "'show" {
                        let content: String = match read_file("./_resources/users.json") {
                                Ok(s) => s,
                                Err(why) => {
                                        println!("Error reading file: {}", why);
                                        String::from("Failed")
                                },
                        };
                        let users: Users = serialize(content);
                        let author: String = message.author.name.to_string();
                        
                        match users.find(&author) {
                                Some(user) => {
                                        let next_mine_upgrade = src::exponant::Exponant::get(&(&user.mine_upgrades + 1.0), 25.0);
                                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                                message.embed(|embed| {
                                                        embed.title("Next upgrade");
                                                        embed.description(format!("The next upgrade for {}", author));
                                                        embed.field(format!("Mine lvl: {}", user.mine_upgrades + 1.0), format!("price: {}", next_mine_upgrade.to_string()), false);
                                                        embed.field("Actual golds", user.golds.to_string(), false);

                                                        return embed;
                                                });

                                                return message;
                                        }) {
                                                println!("Error sending message: {}", why);
                                        }
                                },
                                None => {
                                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                                message.embed(|embed| {
                                                        embed.title("Error !");
                                                        embed.description("I've occured an error\nMaybe you're not registred, do it with `'init`\n\nOr please call the bot's admin");

                                                        return embed;
                                                });

                                                return message;
                                        }) {
                                                println!("Error sending message: {}", why);
                                        }
                                }
                        }
                } else if command[0] == "'engage" {
                        let content: String = match read_file("./_resources/users.json") {
                                Ok(s) => s,
                                Err(why) => {
                                        println!("Error reading file: {}", why);
                                        String::from("failed")
                                }
                        };
                        let mut users: Users = serialize(content);
                        let author: String = message.author.name.to_string();
/*
                        match users.engage_miner(&author) {
                                Ok(()) => {
                                        match users.buy(&author, 10.0) {

                                        }
                                },
                                Err(why) => {

                                }
                        }*/
                } else if command[0] == "'upgrade" {
                        if command[1] == "pickaxe" {
                                let content: String = match read_file("./_resources/users.json") {
                                        Ok(s) => s,
                                        Err(why) => {
                                                println!("Error reading file: {}", why);
                                                String::from("failed")
                                        }
                                };
                                let mut users: Users = serialize(content);
                                let author: String = message.author.name.to_string();

                                

                                match users.upgrade_and_update_mine_ref(&author) {
                                        Ok(()) => {
                                                match users.buy(&author, 25.0) {
                                                        Ok(output) => {
                                                                if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                                                        message.embed(|embed| {
                                                                                embed.title("Upgraded !");
                                                                                embed.description(format!("You've just upgraded your picaxe to the level {}", output));

                                                                                return embed;
                                                                        });

                                                                        return message;
                                                                }) {
                                                                        println!("Error sending message: {}", why);
                                                                }
                                                        },
                                                        Err(why) => {
                                                                println!("Could not buy the upgrade: {}", why);
                                                        },
                                                }
                                        },
                                        Err(output) => {
                                                if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                                        message.embed(|embed| {
                                                                embed.title("Upgraded !");
                                                                embed.description(format!("{}", output));

                                                                return embed;
                                                        });

                                                        return message;
                                                }) {
                                                        println!("Error sending message: {}", why);
                                                }
                                        }
                                }

                                match deserialize_and_write(&users, "./_resources/users.json") {
                                        Ok(()) => (),
                                        Err(why) => {
                                                println!("Error on deserliazing and write users: {}", why);
                                                std::process::exit(1)
                                        }
                                }
                        } else {
                                if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                        message.embed(|embed| {
                                                embed.title("Error !");
                                                embed.description("You've tried to upgrade something which doesn't exists");
                                                embed.field("Upgrade available", "pickaxe, miner", false);

                                                return embed;
                                        });

                                        return message;
                                }) {
                                        println!("Error sending message: {}", why);
                                }
                        }
                } else if command[0] == "'mine" {
                        let content: String = match read_file("./_resources/users.json") {
                                Ok(s) => s,
                                Err(why) => {
                                        println!("Error reading file: {}", why);
                                        String::from("failed")
                                }
                        };
                        let mut users: Users = serialize(content);
                        let author: String = message.author.name.to_string();

                        let user_gold_ref: f64 = match users.find(&author) {
                                Some(user) => user.mine_ref,
                                None => {
                                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                                message.embed(|embed| {
                                                        embed.title("Error !");
                                                        embed.description("I've occured an error\nMaybe you're not registred, do it with `'init`\n\nOr please call the bot's admin");

                                                        return embed;
                                                });

                                                return message;
                                        }) {
                                                println!("Error sending message: {}", why);
                                        }
                                        return;
                                },
                        };

                        let user_last_mine: i64 = match users.get_last_mine(&author) {
                                Some(t) => t,
                                None => time::get_time().sec,
                        };
                        let time_since: i64 = time::get_time().sec - user_last_mine;
                        if time_since >= data::TIME_SINCE_MINE {
                                match users.update_last_mine(&author) {
                                        Ok(()) => (),
                                        Err(why) => {
                                                println!("Could not update the user last_mine: {}", why);
                                        }
                                }

                                let ok: bool = match users.add_golds(&author) {
                                        Ok(()) => true,
                                        Err(why) => {
                                                println!("Error on adding gold to the user: {}", why);
                                                false
                                        }
                                };

                                if ok {
                                        match deserialize_and_write(&users, "./_resources/users.json") {
                                                Ok(()) => (),
                                                Err(why) => {
                                                        println!("Error on deserliazing and write users: {}", why);
                                                        std::process::exit(1)
                                                }
                                        }

                                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                                message.embed(|embed| {
                                                        embed.title("Mined !");
                                                        embed.description(format!("You've just mined `{0}` golds\nplease wait `{1}` seconds before mining again", user_gold_ref, data::TIME_SINCE_MINE));

                                                        return embed;
                                                });

                                                return message;
                                        }) {
                                                println!("Error sending message: {}", why);
                                        }
                                } else {
                                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                                message.embed(|embed| {
                                                        embed.title("Error !");
                                                        embed.description("I've occured an error\nMaybe you're not registred, do it with `'init`\n\nOr please call the bot's admin");

                                                        return embed;
                                                });

                                                return message;
                                        }) {
                                                println!("Error sending message: {}", why);
                                        }
                                }
                        } else {
                                if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                        message.embed(|embed| {
                                                embed.title("Can't mine now");
                                                embed.description(format!("You must wait {} sec before mining again", (time_since - data::TIME_SINCE_MINE).abs()));
                                                return embed;
                                        });
                                        return message;
                                }) {
                                        println!("Error sending message: {}", why);
                                }
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
                        match users.find(&message.author.name.to_string()) {
                                Some(stat) => {
                                        if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                                message.embed(|embed| {
                                                        embed.title(format!("{} stats", stat.name));
                                                        embed.field("Golds", stat.golds.to_string(), false);
                                                        embed.field("Mine value", stat.mine_ref.to_string(), false);
                                                        return embed;
                                                });
                
                                                return message;
                                        }) {
                                                println!("Error sending message: {}", why);
                                        }
                                },
                                None => {
                                        if let Err(why) = message.channel_id.say(&context.http, "You're not playing for now, register with `'init`") {
                                                println!("Error sending message: {}", why);
                                                return;
                                        }
                                }
                        };
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

                        match users.find(&user) {
                                Some(user) => {
                                        if let Err(why) = message.channel_id.say(&context.http, format!("You're already playing as {}", &user.name)) {
                                                println!("Error sending message: {}", why);
                                        }
                                },
                                None => {
                                        if let Err(why) = message.channel_id.say(&context.http, format!("You're now playing as {}", &user)) {
                                                println!("Error sending message: {}", why);
                                        }
                                        users.users.push(User {name: user, golds: 0.0, mine_ref: 1.0, last_mine: 0, mine_upgrades: 1.0, miners: 0});
                                }
                        }

                        match deserialize_and_write(&users, "./_resources/users.json") {
                                Ok(()) => (),
                                Err(why) => {
                                        println!("Error on deserliazing and write users: {}", why);
                                        std::process::exit(1)
                                }
                        }
                }
        }
}

fn deserialize_and_write(users: &Users, path: &str) -> Result<(), String> {
        let json: String = deserialize(&users);
        match write_file(json, path) {
                Ok(()) => return Ok(()),
                Err(why) => {
                        return Err(format!("Failed write file: {}", why));
                        
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
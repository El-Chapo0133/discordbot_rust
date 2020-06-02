

mod src;



use serenity::{
        prelude::*,
        model::prelude::*,
        Client
};


struct Handler {
        prefix: &'static str,
}
impl EventHandler for Handler {
        fn message(&self, context: Context, message: Message) {
                if start_with_prefix(message.content, &self.prefix) {
                        // TODO : 
                        //  lead all bot functions
                        
                        println!("{}", message.author);

                        if let Err(why) = message.channel_id.say(&context.http, "hey!") {
                                println!("Error on seding message: {}", why);
                        }

                        /*if let Err(why) = message.channel_id.send_message(&context.http, |message| {
                                message.embed(|embed| {
                                        embed.title("title");
                                        embed.description("description");
                                        embed.field("name", "value", false);

                                        return embed;
                                });

                                return message;
                        }) {
                                println!("Error sending message: {}", why);
                        }*/
                }
        }
}

fn start_with_prefix(message: String, prefix: &str) -> bool {
        match message.get(0..prefix.len()) {
                Some(s) => return s == prefix,
                None => return false,
        }
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











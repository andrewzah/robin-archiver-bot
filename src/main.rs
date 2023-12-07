mod errors;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use crate::errors::RobinError;

struct Data {}

// scopes: 309237730368
// https://discord.com/oauth2/authorize?client_id=1182146177064517752&scope=bot&permissions=309237763136
#[tokio::main]
async fn main() -> Result<(), RobinError> {
    let token = std::env::var("DISCORD_TOKEN").expect("Is DISCORD_TOKEN set...?");

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client.");

    if let Err(err) = client.start().await {
        println!("client err: #{}", err);
    }

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, msg: Message) {
        println!("msg: [{}]", msg.content);
    }


    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

use std::env;

use std::thread;
use std::sync::Arc;
use std::time::Duration;
use regex::Regex;
use serenity::async_trait;
use serenity::futures::TryFutureExt;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.channel_id != 1000215517673554012 {
            return;
        }

        let ctx = Arc::new(ctx);
        let gateway_raw: &'static str = "This server does not tolerate piracy in any form. I will be banned if I ask for downloads, if I share links or website names for piracy sites, if I admit that I have pirated, or if I advocate for piracy.";
        
        let pattern = Regex::new(r"[*,.-` ><!]").unwrap();
        let clean = pattern.replace_all(&msg.content, "").to_lowercase();
        let gateway = pattern.replace_all(gateway_raw, "").to_lowercase();

        if let Err(why) = msg.delete(&ctx.http).await {
            println!("Couldn't delete message! {:?}", why);
        }

        if String::eq(&clean, &gateway) {
            if let Some(guild_id) = msg.guild_id {
                if let Some(guild) = guild_id.to_guild_cached(&ctx) {
                    let mut member = 
                        guild.member(&ctx, msg.author.id).await.unwrap();

                    if let Err(why) = member.add_role(&ctx, 1000167241465209024).await {
                        println!("Couldn't add role! {:?}", why);
                    }
                } else {
                    println!("Couldn't get guild");
                }
            }
           
            return;
        }

        let incorrect_msg = format!("<@{:?}> To enter the server, please enter: `{}`", msg.author.id.as_u64(), gateway_raw); 
        let future = msg.channel_id.say(&ctx.http, incorrect_msg);


        let ctx1 = Arc::clone(&ctx);

        let then = future.and_then(|msg| async move { 
            thread::sleep(Duration::from_millis(3000));
            _ = msg.delete(&ctx1.http).await;

            return Ok(());
        });

        if let Err(why) = then.await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

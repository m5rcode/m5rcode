/// m5rcode Discord Bridge - Rust backend for discord.m5
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::fs;
use std::process::Command;

struct Handler {
    script_path: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Call m5rcode script with message data
        let json_data = serde_json::json!({
            "event": "message",
            "content": msg.content,
            "author": msg.author.name,
            "author_id": msg.author.id.to_string(),
            "channel_id": msg.channel_id.to_string(),
            "guild_id": msg.guild_id.map(|id| id.to_string()),
        });
        
        // Write event to temp file
        let temp_file = "/tmp/m5rcode_discord_event.json";
        fs::write(temp_file, json_data.to_string()).ok();
        
        // Execute m5rcode handler
        let output = Command::new("m5repl")
            .arg(&self.script_path)
            .env("DISCORD_EVENT_FILE", temp_file)
            .output();
        
        if let Ok(result) = output {
            let response = String::from_utf8_lossy(&result.stdout);
            if !response.trim().is_empty() {
                if let Err(e) = msg.channel_id.say(&ctx.http, response.trim()).await {
                    eprintln!("Error sending message: {:?}", e);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅ {} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: m5rcode-discord <token> <script.m5>");
        std::process::exit(1);
    }
    
    let token = &args[1];
    let script_path = args[2].clone();
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler { script_path })
        .await
        .expect("Error creating client");

    println!("🤖 Starting Discord bot...");
    
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

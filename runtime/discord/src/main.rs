/// m5rcode Discord Bridge - Production-grade implementation
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::fs;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::RwLock;

struct BotState {
    command_handlers: Arc<RwLock<std::collections::HashMap<String, String>>>,
    event_handlers: Arc<RwLock<std::collections::HashMap<String, String>>>,
}

struct Handler {
    state: Arc<BotState>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore bot messages
        if msg.author.bot {
            return;
        }
        
        // Parse command
        let content = msg.content.trim();
        if !content.starts_with('!') {
            return;
        }
        
        let parts: Vec<&str> = content.splitn(2, ' ').collect();
        let command = parts[0].trim_start_matches('!');
        let args = if parts.len() > 1 { parts[1] } else { "" };
        
        // Create event data JSON
        let event_json = serde_json::json!({
            "type": "message",
            "command": command,
            "args": args,
            "content": msg.content,
            "author": {
                "id": msg.author.id.to_string(),
                "name": msg.author.name,
                "discriminator": msg.author.discriminator,
                "bot": msg.author.bot,
            },
            "channel": {
                "id": msg.channel_id.to_string(),
            },
            "guild": msg.guild_id.map(|id| serde_json::json!({
                "id": id.to_string(),
            })),
            "message": {
                "id": msg.id.to_string(),
                "timestamp": msg.timestamp.to_string(),
            }
        });
        
        // Write event to file
        let event_file = "/tmp/m5rcode_discord_event.json";
        if let Err(e) = fs::write(event_file, event_json.to_string()) {
            eprintln!("Error writing event file: {}", e);
            return;
        }
        
        // Execute command handler
        let handlers = self.state.command_handlers.read().await;
        if let Some(handler_script) = handlers.get(command) {
            let output = Command::new("m5repl")
                .arg(handler_script)
                .env("DISCORD_EVENT", event_file)
                .env("DISCORD_COMMAND", command)
                .env("DISCORD_ARGS", args)
                .env("DISCORD_AUTHOR", &msg.author.name)
                .env("DISCORD_CHANNEL", msg.channel_id.to_string())
                .output();
            
            if let Ok(result) = output {
                let stdout = String::from_utf8_lossy(&result.stdout);
                let stderr = String::from_utf8_lossy(&result.stderr);
                
                if !stderr.is_empty() {
                    eprintln!("Handler error: {}", stderr);
                }
                
                let response = stdout.trim();
                if !response.is_empty() {
                    if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
                        eprintln!("Error sending message: {:?}", e);
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅ {} is connected!", ready.user.name);
        println!("📊 Bot ID: {}", ready.user.id);
        println!("🔧 API Version: {}", ready.version);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("m5rcode Discord Bridge v0.1.0");
        eprintln!("\nUsage:");
        eprintln!("  m5rcode-discord <token> <config.json>");
        eprintln!("\nConfig format:");
        eprintln!("  {{");
        eprintln!("    \"commands\": {{");
        eprintln!("      \"ping\": \"handlers/ping.m5\",");
        eprintln!("      \"help\": \"handlers/help.m5\"");
        eprintln!("    }}");
        eprintln!("  }}");
        std::process::exit(1);
    }
    
    let token = &args[1];
    let config_path = if args.len() > 2 { &args[2] } else { "bot_config.json" };
    
    // Load configuration
    let config_content = fs::read_to_string(config_path)
        .unwrap_or_else(|_| r#"{"commands":{}}"#.to_string());
    
    let config: serde_json::Value = serde_json::from_str(&config_content)
        .expect("Invalid config JSON");
    
    // Parse command handlers
    let mut command_handlers = std::collections::HashMap::new();
    if let Some(commands) = config["commands"].as_object() {
        for (cmd, handler) in commands {
            if let Some(handler_path) = handler.as_str() {
                command_handlers.insert(cmd.clone(), handler_path.to_string());
                println!("📝 Registered command: !{} -> {}", cmd, handler_path);
            }
        }
    }
    
    let state = Arc::new(BotState {
        command_handlers: Arc::new(RwLock::new(command_handlers)),
        event_handlers: Arc::new(RwLock::new(std::collections::HashMap::new())),
    });
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler { state })
        .await
        .expect("Error creating client");

    println!("🤖 Starting Discord bot...");
    
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}

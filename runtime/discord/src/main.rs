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

impl Handler {
    fn extract_user_id(mention: &str) -> Option<u64> {
        // Extract user ID from mention like <@123456> or <@!123456>
        let cleaned = mention.trim_start_matches("<@").trim_start_matches("!").trim_end_matches(">");
        cleaned.parse().ok()
    }
    
    async fn handle_api_call(&self, ctx: &Context, msg: &Message, api_line: &str) -> Result<(), Box<dyn std::error::Error>> {
        use serenity::model::id::{UserId, RoleId, ChannelId};
        
        let parts: Vec<&str> = api_line.split(']').collect();
        if parts.len() < 2 {
            return Ok(());
        }
        
        let api_type = parts[0].trim_start_matches("[API:");
        let args_str = parts[1];
        let args: Vec<&str> = args_str.split('|').collect();
        
        match api_type {
            "KICK" => {
                if let (Some(&user_mention), Some(&reason)) = (args.get(0), args.get(1)) {
                    let guild_id = msg.guild_id.ok_or("Not in a guild")?;
                    
                    // Extract user ID from mention or parse directly
                    let user_id = if let Some(id) = Self::extract_user_id(user_mention) {
                        UserId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    ctx.http.kick_member(guild_id, user_id, Some(reason)).await?;
                    msg.channel_id.say(&ctx.http, format!("✅ Kicked <@{}>: {}", user_id, reason)).await?;
                }
            }
            "BAN" => {
                if let (Some(&user_mention), Some(&reason), Some(&days_str)) = (args.get(0), args.get(1), args.get(2)) {
                    let guild_id = msg.guild_id.ok_or("Not in a guild")?;
                    
                    let user_id = if let Some(id) = Self::extract_user_id(user_mention) {
                        UserId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    let days: u8 = days_str.parse().unwrap_or(0);
                    ctx.http.ban_user(guild_id, user_id, days, Some(reason)).await?;
                    msg.channel_id.say(&ctx.http, format!("🔨 Banned <@{}>: {}", user_id, reason)).await?;
                }
            }
            "TIMEOUT" => {
                if let (Some(&user_mention), Some(&duration_str), Some(&reason)) = (args.get(0), args.get(1), args.get(2)) {
                    let guild_id = msg.guild_id.ok_or("Not in a guild")?;
                    
                    let user_id = if let Some(id) = Self::extract_user_id(user_mention) {
                        UserId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    let duration_secs: i64 = duration_str.parse()?;
                    
                    use serenity::model::Timestamp;
                    
                    let mut member = ctx.http.get_member(guild_id, user_id).await?;
                    let timeout_until = Timestamp::from_unix_timestamp(
                        chrono::Utc::now().timestamp() + duration_secs
                    )?;
                    
                    member.disable_communication_until_datetime(&ctx.http, timeout_until).await?;
                    msg.channel_id.say(&ctx.http, format!("🔇 Muted <@{}> for {}s: {}", user_id, duration_secs, reason)).await?;
                }
            }
            "UNTIMEOUT" => {
                if let Some(&user_mention) = args.get(0) {
                    let guild_id = msg.guild_id.ok_or("Not in a guild")?;
                    
                    let user_id = if let Some(id) = Self::extract_user_id(user_mention) {
                        UserId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    let mut member = ctx.http.get_member(guild_id, user_id).await?;
                    member.enable_communication(&ctx.http).await?;
                    msg.channel_id.say(&ctx.http, format!("🔊 Unmuted <@{}>", user_id)).await?;
                }
            }
            "UNBAN" => {
                if let (Some(&user_mention), Some(&reason)) = (args.get(0), args.get(1)) {
                    let guild_id = msg.guild_id.ok_or("Not in a guild")?;
                    
                    let user_id = if let Some(id) = Self::extract_user_id(user_mention) {
                        UserId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    ctx.http.remove_ban(guild_id, user_id, Some(reason)).await?;
                    msg.channel_id.say(&ctx.http, format!("✅ Unbanned user: {}", reason)).await?;
                }
            }
            "ADD_ROLE" => {
                if let (Some(&user_mention), Some(&role_mention), Some(&reason)) = (args.get(0), args.get(1), args.get(2)) {
                    let guild_id = msg.guild_id.ok_or("Not in a guild")?;
                    
                    let user_id = if let Some(id) = Self::extract_user_id(user_mention) {
                        UserId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    // Extract role ID from <@&123> or parse directly
                    let role_id = if let Some(id) = role_mention.trim_start_matches("<@&").trim_end_matches(">").parse().ok() {
                        RoleId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    let mut member = ctx.http.get_member(guild_id, user_id).await?;
                    member.add_role(&ctx.http, role_id).await?;
                    msg.channel_id.say(&ctx.http, format!("➕ Added role <@&{}> to <@{}>", role_id, user_id)).await?;
                }
            }
            "REMOVE_ROLE" => {
                if let (Some(&user_mention), Some(&role_mention), Some(&reason)) = (args.get(0), args.get(1), args.get(2)) {
                    let guild_id = msg.guild_id.ok_or("Not in a guild")?;
                    
                    let user_id = if let Some(id) = Self::extract_user_id(user_mention) {
                        UserId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    let role_id = if let Some(id) = role_mention.trim_start_matches("<@&").trim_end_matches(">").parse().ok() {
                        RoleId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    let mut member = ctx.http.get_member(guild_id, user_id).await?;
                    member.remove_role(&ctx.http, role_id).await?;
                    msg.channel_id.say(&ctx.http, format!("➖ Removed role <@&{}> from <@{}>", role_id, user_id)).await?;
                }
            }
            "REACT" => {
                if let Some(&emoji) = args.get(0) {
                    use serenity::model::channel::ReactionType;
                    if let Ok(reaction) = emoji.parse::<ReactionType>() {
                        msg.react(&ctx.http, reaction).await?;
                    }
                }
            }
            "SEND_DM" => {
                if let (Some(&user_mention), Some(&message)) = (args.get(0), args.get(1)) {
                    let user_id = if let Some(id) = Self::extract_user_id(user_mention) {
                        UserId::new(id)
                    } else {
                        return Ok(());
                    };
                    
                    user_id.create_dm_channel(&ctx.http).await?.say(&ctx.http, message).await?;
                }
            }
            "SLOWMODE" => {
                if let (Some(&channel_str), Some(&seconds_str)) = (args.get(0), args.get(1)) {
                    let channel_id = if channel_str == "CURRENT_CHANNEL" {
                        msg.channel_id
                    } else {
                        ChannelId::new(channel_str.parse()?)
                    };
                    
                    let seconds: u16 = seconds_str.parse().unwrap_or(0);
                    
                    use serenity::builder::EditChannel;
                    channel_id.edit(&ctx.http, EditChannel::new().rate_limit_per_user(seconds)).await?;
                    
                    if seconds == 0 {
                        msg.channel_id.say(&ctx.http, "⏱️ Slowmode disabled").await?;
                    } else {
                        msg.channel_id.say(&ctx.http, format!("⏱️ Slowmode: {}s", seconds)).await?;
                    }
                }
            }
            "EMBED" => {
                if let (Some(&title), Some(&description), Some(&color_str)) = (args.get(0), args.get(1), args.get(2)) {
                    use serenity::builder::{CreateEmbed, CreateMessage};
                    
                    let color: u32 = color_str.parse().unwrap_or(0x3498db);
                    let embed = CreateEmbed::new()
                        .title(title)
                        .description(description)
                        .color(color);
                    
                    msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await?;
                }
            }
            "DELETE_MESSAGES" => {
                if let (Some(&channel_str), Some(&count_str)) = (args.get(0), args.get(1)) {
                    let channel_id = if channel_str == "CURRENT_CHANNEL" {
                        msg.channel_id
                    } else {
                        ChannelId::new(channel_str.parse()?)
                    };
                    
                    let count: u8 = count_str.parse().unwrap_or(10).min(100);
                    
                    let messages = channel_id.messages(&ctx.http, serenity::builder::GetMessages::new().limit(count)).await?;
                    
                    if !messages.is_empty() {
                        if messages.len() == 1 {
                            messages[0].delete(&ctx.http).await?;
                        } else {
                            channel_id.delete_messages(&ctx.http, &messages).await?;
                        }
                        msg.channel_id.say(&ctx.http, format!("🗑️ Deleted {} messages", messages.len())).await?;
                    }
                }
            }
            _ => {}
        }
        
        Ok(())
    }
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
            // Create a wrapper script that injects command variables at the top
            let wrapper = format!(
                "let DISCORD_COMMAND = \"{}\"\nlet DISCORD_ARGS = \"{}\"\nlet DISCORD_AUTHOR = \"{}\"\n\n",
                command, 
                args.replace("\"", "\\\"").replace("\n", "\\n"), 
                msg.author.name.replace("\"", "\\\"")
            );
            
            // Read the handler script
            let handler_content = match fs::read_to_string(handler_script) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading handler {}: {}", handler_script, e);
                    return;
                }
            };
            
            // Remove any existing DISCORD_* variable definitions from the handler
            let mut filtered_content = String::new();
            for line in handler_content.lines() {
                if !line.trim().starts_with("let DISCORD_COMMAND") 
                    && !line.trim().starts_with("let DISCORD_ARGS")
                    && !line.trim().starts_with("let DISCORD_AUTHOR") {
                    filtered_content.push_str(line);
                    filtered_content.push('\n');
                }
            }
            
            // Write combined script
            let wrapper_file = "/tmp/m5rcode_discord_wrapper.m5";
            let full_script = format!("{}{}", wrapper, filtered_content);
            if let Err(e) = fs::write(wrapper_file, full_script) {
                eprintln!("Error writing wrapper: {}", e);
                return;
            }
            
            let output = Command::new("m5repl")
                .arg(wrapper_file)
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
                
                // Process API calls and regular responses
                for line in stdout.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    
                    // Handle API calls
                    if line.starts_with("[API:") {
                        if let Err(e) = self.handle_api_call(&ctx, &msg, line).await {
                            eprintln!("API call error: {}", e);
                        }
                    } else {
                        // Regular message response
                        if let Err(e) = msg.channel_id.say(&ctx.http, line).await {
                            eprintln!("Error sending message: {:?}", e);
                        }
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
        eprintln!("m5rcode Discord Bridge v0.2.0");
        eprintln!("\nUsage:");
        eprintln!("  m5rcode-discord <token> <bot.m5>           # Single-file bot");
        eprintln!("  m5rcode-discord <token> <config.json>      # Multi-file bot");
        std::process::exit(1);
    }
    
    let token = &args[1];
    let bot_file = if args.len() > 2 { &args[2] } else { "bot.m5" };
    
    // Check if it's a JSON config or m5 file
    let is_single_file = bot_file.ends_with(".m5");
    
    let command_handlers = if is_single_file {
        // Single-file mode: all commands in one file
        println!("📝 Single-file bot mode: {}", bot_file);
        let mut handlers = std::collections::HashMap::new();
        
        // Register all commands to use the same file
        for cmd in &[
            "ping", "hello", "info", "calc", "help", "math",
            "kick", "ban", "unban", "mute", "unmute", 
            "warn", "warnings", "clearwarns",
            "purge", "slowmode", "lock", "unlock",
            "userinfo", "serverinfo", "modlogs",
            "addrole", "removerole", "automod"
        ] {
            handlers.insert(cmd.to_string(), bot_file.to_string());
            println!("📝 Registered command: !{} -> {}", cmd, bot_file);
        }
        handlers
    } else {
        // Multi-file mode: load from config
        let config_content = fs::read_to_string(bot_file)
            .unwrap_or_else(|_| r#"{"commands":{}}"#.to_string());
        
        let config: serde_json::Value = serde_json::from_str(&config_content)
            .expect("Invalid config JSON");
        
        let mut handlers = std::collections::HashMap::new();
        if let Some(commands) = config["commands"].as_object() {
            for (cmd, handler) in commands {
                if let Some(handler_path) = handler.as_str() {
                    handlers.insert(cmd.clone(), handler_path.to_string());
                    println!("📝 Registered command: !{} -> {}", cmd, handler_path);
                }
            }
        }
        handlers
    };
    
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

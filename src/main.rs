extern crate rust_bert;
use rust_bert::pipelines::conversation::{ConversationManager, ConversationModel};
use std::error::Error;

use dotenv::dotenv;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    setup().await;
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let msg_content = &msg.content;
    // msg.reply(ctx, msg_content).await?;
    let _ = gr(msg_content, ctx, msg).await;

    Ok(())
}

async fn gr(input: &str, ctx: &Context, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
    let conversation_model = ConversationModel::new(Default::default())?;
    let mut conversation_manager = ConversationManager::new();

    // Create a conversation with an initial message
    let conversation_id = conversation_manager.create(input);

    // Generate responses for the conversation
    let output = conversation_model.generate_responses(&mut conversation_manager);

    // Print the responses (you can customize how you handle the responses here)
    for response in output.iter() {
        let _ = msg.reply(ctx, response.1.to_owned()).await;
        println!("Response: {:?}", response);
    }
    Ok(())
}

fn get_response(
    input: &String,
    msg: &Message,
    ctx: &Context,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the conversation model
    msg.reply(ctx, "test");
    let conversation_model = ConversationModel::new(Default::default())?;
    let mut conversation_manager = ConversationManager::new();

    // Create a conversation with an initial message
    let conversation_id = conversation_manager.create(input);

    // Generate responses for the conversation
    let output = conversation_model.generate_responses(&mut conversation_manager);

    // Print the responses (you can customize how you handle the responses here)
    for response in output.iter() {
        println!("Response: {:?}", response);
    }
    Ok(())
}
async fn setup() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-"))
        .group(&GENERAL_GROUP);

    dotenv().unwrap();
    let token = std::env::var("DISCORD_TOKEN").expect("token");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

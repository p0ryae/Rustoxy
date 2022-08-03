use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::Ready;
use serenity::builder::CreateEmbed;
use serenity::prelude::*;
use std::env;
use std::time::Instant;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "\x1b[1;32m[ONLINE] \x1b[0;31mConnected as {}\x1b[0m",
            ready.user.tag()
        );
        let _slashcommand = Command::create_global_application_command(&ctx.http, |command| {
            command.name("bins").description("List code bins")
        })
        .await;
        let _ = Command::create_global_application_command(&ctx.http, |command| {
            command.name("ping").description("Shows latency")
        })
        .await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(ref command) = interaction {
            let msg_interaction = interaction.clone().application_command().unwrap();
            let _content = match command.data.name.as_str() {
                "ping" => {
                    msg_interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response.interaction_response_data(|message| {
                                message.content("Pong!").ephemeral(false)
                            })
                        })
                        .await
                }
                "bins" => {
                    let mut embed = CreateEmbed::default();
                    embed.title("Paste your code in one of the following websites");
                    embed.description("> [__`SourceBin`__](https://sourceb.in/)
                    > [__`HasteBin`__](https://hasteb.in/)
                    > [__`HateBin`__](https://hatebin.com/)
                    > [__`Hastebin`__](https://hastebin.com/)
                    > [__`PasteBin`__](https://pastebin.com/)");
                    embed.color((47, 49, 54));
                    embed.footer(|f| {f.text("Just use SourceBin for bobux 😳")});

                    msg_interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response.interaction_response_data(|message| {
                                message.add_embed(embed).ephemeral(false)
                            })
                        })
                        .await
                }
                _ => {
                    msg_interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response.interaction_response_data(|message| {
                                message.content("Something went wrong.").ephemeral(true)
                            })
                        })
                        .await
                }
            };
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let before = Instant::now();
    let mut message = msg.reply(&ctx.http, "pong!").await?;
    let after = Instant::now();

    let content = message.content.clone();
    message
        .edit(ctx, |m| {
            m.content(format!(
                "{} - `{}ms`",
                content,
                (after - before).as_millis()
            ))
        })
        .await?;

    Ok(())
}

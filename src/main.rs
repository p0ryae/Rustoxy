use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use std::env;
use std::time::Instant;
use rand::Rng; 

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
                    embed.description(
                        "> [__`SourceBin`__](https://sourceb.in/)
                    > [__`HasteBin`__](https://hasteb.in/)
                    > [__`HateBin`__](https://hatebin.com/)
                    > [__`Hastebin`__](https://hastebin.com/)
                    > [__`PasteBin`__](https://pastebin.com/)",
                    );
                    embed.color((47, 49, 54));
                    embed.footer(|f| f.text("Just use SourceBin for bobux 😳"));

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
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let guild_id = new_member.guild_id;
        if let Ok(guild) = guild_id.to_partial_guild(&ctx).await {
            let channels = guild.channels(&ctx).await.unwrap();
            let channel_search = channels.values().find(|c| c.name == "general");
            if let Some(channel) = channel_search {
                let num = rand::thread_rng().gen_range(0..=255);

                let _ = channel
                    .send_message(&ctx, |m| {
                        m.embed(|e| {
                            if let Some(ref joined_at) = new_member.joined_at {
                                e.timestamp(joined_at);
                            }
                            e.description(format!(
                                "**{}**, joined DashCruft Nation!",
                                new_member.display_name()
                            ));
                            e.color((num, num, num));
                            e.footer(|f| f.text("Welcome!"))

                        })
                    })
                    .await;
            }
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
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS;
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

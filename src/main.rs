use rand::Rng;
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{Ready, GuildId};
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
        let _binslash = Command::create_global_application_command(&ctx.http, |command| {
            command.name("bins").description("List code bins")
        })
        .await;
        let _pingslash = Command::create_global_application_command(&ctx.http, |command| {
            command.name("ping").description("Shows latency")
        })
        .await;
        let _muteslash = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("mute")
                .description("Mute a user in the guild")
                .create_option(|option| {
                    option
                        .name("user")
                        .description("The user you'd like to mute")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
        .await;
        let _unmuteslash = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("unmute")
                .description("Unmute a user in the guild")
                .create_option(|option| {
                    option
                        .name("user")
                        .description("The user you'd like to unmute")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
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
                "mute" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let CommandDataOptionValue::User(user, _member) = options {
                        let guild_id: GuildId = serenity::model::id::GuildId(644764850706448384);
                        guild_id
                            .member(&ctx.http, user.id)
                            .await
                            .unwrap()
                            .add_role(&ctx, 661684608034799637)
                            .await
                            .unwrap();

                        guild_id
                            .member(&ctx.http, user.id)
                            .await
                            .unwrap()
                            .remove_role(&ctx, 775603112207056916)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content(format!(
                                            "Favorably, {}'s mute resulted in `true`!",
                                            user
                                        ))
                                        .ephemeral(false)
                                })
                            })
                            .await
                    } else {
                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Please provide a valid user.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    }
                }
                "unmute" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let CommandDataOptionValue::User(user, _member) = options {
                        use serenity::model::prelude::GuildId;
                        let guild_id: GuildId = serenity::model::id::GuildId(644764850706448384);
                        guild_id
                            .member(&ctx.http, user.id)
                            .await
                            .unwrap()
                            .remove_role(&ctx, 661684608034799637)
                            .await
                            .unwrap();

                        guild_id
                            .member(&ctx.http, user.id)
                            .await
                            .unwrap()
                            .add_role(&ctx, 775603112207056916)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content(format!(
                                            "Favorably, {}'s unmute resulted in `true`!",
                                            user
                                        ))
                                        .ephemeral(false)
                                })
                            })
                            .await
                    } else {
                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Please provide a valid user.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    }
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
            new_member
                .clone()
                .add_role(&ctx, 775603112207056916)
                .await
                .unwrap();

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

    dotenv::dotenv().expect("Failed to load .env");

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

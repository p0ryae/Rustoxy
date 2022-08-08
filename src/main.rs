mod commands;
use crate::commands::cmds::*;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::component::{ActionRowComponent, InputText};
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::guild::Member;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{ChannelId, GuildId, Ready};
use serenity::prelude::*;

use rand::Rng;
use std::env;

#[group]
#[commands(ping, roles)]
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
        let _reportslash = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("report")
                .description("Report a user")
                .create_option(|option| {
                    option
                        .name("user")
                        .description("The user you'd like to report")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
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
        if let Interaction::MessageComponent(ref command) = interaction {
            let msg_interaction = interaction.clone().message_component().unwrap();
            let _content = match command.data.custom_id.as_str() {
                "button_homie" => {
                    let has_role = if let Ok(e) = interaction
                        .clone()
                        .message_component()
                        .unwrap()
                        .user
                        .has_role(&ctx.http, 644764850706448384, 798793150905974844)
                        .await
                    {
                        e
                    } else {
                        false
                    };
                    if has_role {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .remove_role(&ctx, 798793150905974844)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Detached the **Homies** role.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    } else {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .add_role(&ctx, 798793150905974844)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Added the **Homies** role.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    }
                }
                "button_updates" => {
                    let has_role = if let Ok(e) = interaction
                        .clone()
                        .message_component()
                        .unwrap()
                        .user
                        .has_role(&ctx.http, 644764850706448384, 813307957411971082)
                        .await
                    {
                        e
                    } else {
                        false
                    };
                    if has_role {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .remove_role(&ctx, 813307957411971082)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Detached the **Updates** role.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    } else {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .add_role(&ctx, 813307957411971082)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Added the **Updates** role.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    }
                }
                "button_windows" => {
                    let has_role = if let Ok(e) = interaction
                        .clone()
                        .message_component()
                        .unwrap()
                        .user
                        .has_role(&ctx.http, 644764850706448384, 923446403181735978)
                        .await
                    {
                        e
                    } else {
                        false
                    };
                    if has_role {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .remove_role(&ctx, 923446403181735978)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Detached the **Windows** role.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    } else {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .add_role(&ctx, 923446403181735978)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Added the **Windows** role.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    }
                }
                "button_linux" => {
                    let has_role = if let Ok(e) = interaction
                        .clone()
                        .message_component()
                        .unwrap()
                        .user
                        .has_role(&ctx.http, 644764850706448384, 923446319610232852)
                        .await
                    {
                        e
                    } else {
                        false
                    };
                    if has_role {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .remove_role(&ctx, 923446319610232852)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Detached the **Linux** role.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    } else {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .add_role(&ctx, 923446319610232852)
                            .await
                            .unwrap();

                        interaction
                            .clone()
                            .message_component()
                            .unwrap()
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message.content("Added the **Linux** role.").ephemeral(true)
                                })
                            })
                            .await
                    }
                }
                "button_apple" => {
                    let has_role = if let Ok(e) = interaction
                        .clone()
                        .message_component()
                        .unwrap()
                        .user
                        .has_role(&ctx.http, 644764850706448384, 923446471779553280)
                        .await
                    {
                        e
                    } else {
                        false
                    };
                    if has_role {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .remove_role(&ctx, 923446471779553280)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("Detached the **MacOS** role.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    } else {
                        GuildId(644764850706448384)
                            .member(
                                &ctx.http,
                                interaction.clone().message_component().unwrap().user,
                            )
                            .await
                            .unwrap()
                            .add_role(&ctx, 923446471779553280)
                            .await
                            .unwrap();

                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message.content("Added the **MacOS** role.").ephemeral(true)
                                })
                            })
                            .await
                    }
                }
                _ => {
                    msg_interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response.interaction_response_data(|message| {
                                message
                                    .content("Couldn't identify MessageComponent.")
                                    .ephemeral(true)
                            })
                        })
                        .await
                }
            };
        }

        if let Interaction::ModalSubmit(ref command) = interaction {
            let msg_interaction = interaction.clone().modal_submit().unwrap();
            let _content = match command.data.custom_id.as_str() {
                "report_modal" => {
                    msg_interaction
                        .create_interaction_response(&ctx.http, |response| {
                            response.interaction_response_data(|message| {
                                message.content("Effectively **Reported**. Our Staff Team will look into this ASAP.").ephemeral(true)
                            })
                        })
                        .await.ok();

                    if let ActionRowComponent::InputText(InputText {
                        kind: _,
                        custom_id: _,
                        value,
                    }) = &command.data.components[0].components[0]
                    {
                        ChannelId(1006037491779907714)
                            .send_message(&ctx, |m| {
                                m.embed(|e| {
                                    e.title(msg_interaction.user.tag());
                                    e.description(format!("**INFO**: {}", value));
                                    e.color((47, 49, 54))
                                })
                            })
                            .await
                            .ok();
                    }
                }
                _ => {
                    ChannelId(1006037491779907714)
                        .send_message(&ctx.http, |m| m.content("A report has failed to arrive."))
                        .await
                        .ok();
                }
            };
        }

        if let Interaction::ApplicationCommand(ref command) = interaction {
            let msg_interaction = interaction.clone().application_command().unwrap();
            let _content = match command.data.name.as_str() {
                "report" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let CommandDataOptionValue::User(user, _member) = options {
                        msg_interaction.create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::Modal)
                            .interaction_response_data(|message| {
                                message
                                    .components(|components| {
                                        components.create_action_row(|row| {
                                            row.create_input_text(|b| {
                                                b.label("reason (state the person as well)");
                                                b.style(serenity::model::application::component::InputTextStyle::Paragraph);
                                                b.custom_id("reason_modal_input")
                                            })
                                        })
                                    })
                                    .title(format!("Report {}", user.name))
                                    .custom_id("report_modal")
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
                    let mut embed = serenity::builder::CreateEmbed::default();
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
                    let has_permission = GuildId(644764850706448384)
                        .member(&ctx.http, &msg_interaction.user)
                        .await
                        .unwrap()
                        .permissions(&ctx)
                        .unwrap()
                        .contains(serenity::model::Permissions::MANAGE_ROLES);

                    if has_permission {
                        let options = command
                            .data
                            .options
                            .get(0)
                            .expect("Expected user option")
                            .resolved
                            .as_ref()
                            .expect("Expected user object");

                        if let CommandDataOptionValue::User(user, _member) = options {
                            let guild_id: GuildId =
                                serenity::model::id::GuildId(644764850706448384);
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
                    } else {
                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("You're prohibited from using this command.")
                                        .ephemeral(true)
                                })
                            })
                            .await
                    }
                }
                "unmute" => {
                    let has_permission = GuildId(644764850706448384)
                        .member(&ctx.http, &msg_interaction.user)
                        .await
                        .unwrap()
                        .permissions(&ctx)
                        .unwrap()
                        .contains(serenity::model::Permissions::MANAGE_ROLES);
                    if has_permission {
                        let options = command
                            .data
                            .options
                            .get(0)
                            .expect("Expected user option")
                            .resolved
                            .as_ref()
                            .expect("Expected user object");

                        if let CommandDataOptionValue::User(user, _member) = options {
                            GuildId(644764850706448384)
                                .member(&ctx.http, user.id)
                                .await
                                .unwrap()
                                .remove_role(&ctx, 661684608034799637)
                                .await
                                .unwrap();

                            GuildId(644764850706448384)
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
                    } else {
                        msg_interaction
                            .create_interaction_response(&ctx.http, |response| {
                                response.interaction_response_data(|message| {
                                    message
                                        .content("You're prohibited from using this command.")
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

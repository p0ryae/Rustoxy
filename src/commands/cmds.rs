use serenity::model::channel::Message;
use std::time::Instant;
use serenity::model::application::component::ButtonStyle;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::prelude::*;

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

#[command]
async fn roles(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |message| {
            message
                .add_embed(|embed| {
                    embed
                        .description(
                            "😈 **- Dash's Homies**\n`-` DashCruft uses this role to ping for special news, suggestions, leaks or just for fun.\n\n🌐 **-  Updates**\n`-` Stay tuned with the latest updates regarding DashCruft's projects and such\n\n🪟🐧🍎 **-  Operating System**\n`-` What is your operating system? Windows, Linux or MacOS (Apple)",
                        )
                        .color((47, 49, 54))
                })
                .components(|components| {
                    components.create_action_row(|action_row| {
                        action_row.create_button(|button| {
                            button
                                .custom_id("button_homie")
                                .emoji('😈')
                                .style(ButtonStyle::Primary)
                        });
                        action_row.create_button(|button| {
                            button
                                .custom_id("button_updates")
                                .emoji('🌐')
                                .style(ButtonStyle::Primary)
                        });
                        action_row.create_button(|button| {
                            button
                                .custom_id("button_windows")
                                .emoji('🪟')
                                .style(ButtonStyle::Primary)
                        });
                        action_row.create_button(|button| {
                            button
                                .custom_id("button_linux")
                                .emoji('🐧')
                                .style(ButtonStyle::Primary)
                        });
                        action_row.create_button(|button| {
                            button
                                .custom_id("button_apple")
                                .emoji('🍎')
                                .style(ButtonStyle::Primary)
                        })
                    })
                })
        })
        .await?;

    Ok(())
}
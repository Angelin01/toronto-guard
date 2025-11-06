use poise::serenity_prelude as serenity;
use serenity::EditMember;
use std::time::Duration;
use poise::Command;
use tokio::time::sleep;
use crate::bot::{BotError, BotState, Context};

pub fn commands() -> Vec<Command<BotState, BotError>> {
    let commands = vec![
        disconnect_mason(),
    ];

    commands
}

#[poise::command(
    slash_command,
    rename = "disconnect-mason",
    required_bot_permissions = "MOVE_MEMBERS",
    user_cooldown = 30,
)]
pub async fn disconnect_mason(ctx: Context<'_>) -> Result<(), BotError> {
    let mason_user_id = ctx.data().mason_user_id.as_ref();

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.reply("This command can only be used inside a server.").await?;
            return Ok(());
        }
    };

    let mason_member = match guild_id
        .member(ctx.serenity_context(), mason_user_id)
        .await {
        Ok(member) => member,
        Err(why) => {
            ctx.reply("I failed to find Mason. Either he is not in this guild or I don't have permissions to list members").await?;
            return Err(why.into());
        }
    };

    if let Err(err) = mason_member.disconnect_from_voice(ctx).await {
        ctx.reply("I couldn't disconnect Mason. maybe he's not in a voice channel or I lack permissions.")
            .await?;
        return Err(err.into());
    }

    ctx.reply("Disconnected Mason from voice.").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "mute-mason",
    required_bot_permissions = "MUTE_MEMBERS",
    user_cooldown = 30,
)]
pub async fn mute_mason(ctx: Context<'_>) -> Result<(), BotError> {
    let mason_user_id = ctx.data().mason_user_id.as_ref();

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.reply("This command can only be used inside a server.").await?;
            return Ok(());
        }
    };

    let mut mason_member = match guild_id.member(ctx.serenity_context(), mason_user_id).await {
        Ok(member) => member,
        Err(why) => {
            ctx.reply("I failed to find Mason. Either he is not in this guild or I don't have permissions to list members.").await?;
            return Err(why.into());
        }
    };

    let muter = EditMember::new().mute(true);
    if let Err(err) = mason_member.edit(ctx, muter).await {
        ctx.reply("Couldn't mute Mason, maybe he's not in a voice channel or I lack permissions.")
            .await?;
        return Err(err.into());
    }

    ctx.reply("Muted Mason for 5 seconds.").await?;
    sleep(Duration::from_secs(5)).await;

    let unmuter = EditMember::new().mute(false);
    if let Err(err) = mason_member.edit(ctx, unmuter).await {
        ctx.reply("Failed to unmute Mason after 5 seconds.").await?;
        return Err(err.into());
    }

    ctx.reply("Unmuted Mason.").await?;
    Ok(())
}

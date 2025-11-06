use crate::bot::{BotError, BotState, Context};
use poise::serenity_prelude as serenity;
use poise::Command;
use serenity::EditMember;
use std::time::Duration;
use tokio::time::sleep;

pub fn commands() -> Vec<Command<BotState, BotError>> {
    vec![disconnect_mason(), mute_mason()]
}

async fn get_mason_member(
    ctx: Context<'_>,
) -> Result<(serenity::GuildId, serenity::Member), BotError> {
    let mason_user_id = ctx.data().mason_user_id.as_ref();

    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.reply("This command can only be used inside a server.")
                .await?;
            return Err("Not in a guild".into());
        }
    };

    let mason_member = guild_id
        .member(ctx.serenity_context(), mason_user_id)
        .await?;

    Ok((guild_id, mason_member))
}

#[poise::command(
    slash_command,
    rename = "disconnect-mason",
    required_bot_permissions = "MOVE_MEMBERS",
    user_cooldown = 30
)]
pub async fn disconnect_mason(ctx: Context<'_>) -> Result<(), BotError> {
    let (guild_id, mason_member) = match get_mason_member(ctx).await {
        Ok(pair) => pair,
        Err(_) => {
            ctx.reply("I couldn't find Mason or don't have permission.")
                .await?;
            return Ok(());
        }
    };

    let is_in_voice = ctx
        .guild()
        .expect("Should be in a guild")
        .voice_states
        .get(&ctx.author().id)
        .is_some();
    if !is_in_voice {
        ctx.reply("Mason is not in a voice channel.").await?;
        return Ok(());
    }

    if let Err(err) = mason_member.disconnect_from_voice(ctx).await {
        ctx.reply("I couldn't disconnect Mason. Maybe I lack permissions.")
            .await?;
        return Err(err.into());
    }

    if let Err(err) = mason_member.disconnect_from_voice(ctx).await {
        ctx.reply("I couldn't disconnect Mason. Maybe I lack permissions.")
            .await?;
        return Err(err.into());
    }

    ctx.reply(format!("Disconnected Mason from guild `{guild_id}`."))
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "mute-mason",
    required_bot_permissions = "MUTE_MEMBERS",
    user_cooldown = 30
)]
pub async fn mute_mason(ctx: Context<'_>) -> Result<(), BotError> {
    let (_, mut mason_member) = match get_mason_member(ctx).await {
        Ok(pair) => pair,
        Err(_) => {
            ctx.reply("I couldn't find Mason or don't have permission.")
                .await?;
            return Ok(());
        }
    };

    if let Err(err) = mason_member.edit(ctx, EditMember::new().mute(true)).await {
        ctx.reply("Couldn't mute Mason. Maybe I lack permissions.")
            .await?;
        return Err(err.into());
    }

    ctx.reply("Muted Mason for 5 seconds.").await?;
    sleep(Duration::from_secs(5)).await;

    if let Err(err) = mason_member.edit(ctx, EditMember::new().mute(false)).await {
        ctx.reply("Failed to unmute Mason after 5 seconds. Somebody better help him.")
            .await?;
        return Err(err.into());
    }

    ctx.reply("Unmuted Mason.").await?;
    Ok(())
}

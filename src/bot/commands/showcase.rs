use crate::bot::{Context, Error};
//use poise::{serenity_prelude::CreateEmbed, CreateReply};

struct Publication {
    project_name: String,
}

#[poise::command(
    slash_command,
    hide_in_help = true,
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn showcase(
    ctx: Context<'_>,
    #[description = "Nombre del proyecto"] name: String,
    #[description = "URL al repositorio del proyecto"] url: String,
    #[description = "Descripción del proyecto"] desc: Option<String>,
    #[description = "Tags"] tags: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    ctx.say("Hola").await?;

    Ok(())
}

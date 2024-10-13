use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn msync(ctx: Context<'_>) -> Result<(), Error> {
    let mut message = "### Steam download stops?
Change MSync to ESync in `Bottle Configuration` -> `Enhanced Sync`".to_owned();

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += "\n\nThis command was invoked by ";
                message += ctx.author().to_string().as_str();

                parent.reply_ping(&ctx, message).await?;
                prefix.msg.delete(ctx).await?;
            }
            None => {
                ctx.reply(message).await?;
            }
        }
    } else {
        ctx.reply(message).await?;
    }
    Ok(())
}


mod player;

use std::{
    collections::HashSet,
    env,
};
use std::any::Any;
use std::borrow::Borrow;

use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    framework::standard::{
        Args,
        CommandGroup,
        CommandResult,
        help_commands,
        HelpOptions,
        macros::{command, group, help},
    },
    http::Http,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
    utils::{content_safe, ContentSafeOptions},
};
use serenity::framework::StandardFramework;
use serenity::futures::StreamExt;
use serenity::model::guild::Member;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[summary = "General"]
#[description = "General commands."]
#[commands(hello, channel_member)]
struct General;

#[command]
#[description = "say hello."]
async fn hello(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "hello").await?;
    Ok(())
}

#[command]
#[aliases("member")]
async fn channel_member(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut entry_users: Vec<String> = msg.content.split(" ").map(|split_message| split_message.to_string()).collect();
    let users = entry_users.split_off(1);
    let game_players = player::get_player(users, ctx, msg).await;
    for game_player in game_players {
        println!("{} is a game player.", game_player);
    }
    msg.channel_id.say(&ctx.http, "see logs.").await?;
    Ok(())
}

#[help]
#[individual_command_tip = "this is help command."]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix("~")
            .on_mention(Some(bot_id))
            .owners(owners)
        )
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);
    let mut client =
        Client::builder(&token)
            .event_handler(Handler)
            .framework(framework)
            .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_PRESENCES)
            .await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

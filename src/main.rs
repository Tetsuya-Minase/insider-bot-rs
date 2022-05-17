mod insider_game;

use std::{
    collections::HashSet,
    env,
};

use serenity::{async_trait, framework::standard::{
    Args,
    CommandGroup,
    CommandResult,
    help_commands,
    HelpOptions,
    macros::{command, group, help},
}, http::Http, model::{channel::Message, gateway::Ready, id::UserId}, prelude::*};
use serenity::framework::StandardFramework;

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
#[commands(play_insider_game)]
struct General;

#[command]
#[aliases("insider")]
async fn play_insider_game(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let players = msg.mentions.to_vec();
    if players.len() < 4 {
        msg.channel_id.say(&ctx.http, "At least 4 players are required.").await?;
        Ok(())
    }
    let option_theme = insider_game::get_theme();
    let theme = match option_theme {
        Some(theme) => theme,
        None => {
            msg.channel_id.say(&ctx.http, "Error!! cannot retrieve a theme.").await?;
            Ok(())
        }
    };
    let player_role_list = insider_game::hand_out_role(players);
    for player_role in player_role_list {
        let message = match &*player_role.role {
            "マスター" | "インサイダー" => format!("Hello {}.\nYour role is {}.\nThe theme is {}.", player_role.player_name, player_role.role, theme),
            "市民" => format!("Hello {}.\nYour role is {}.", player_role.player_name, player_role.role),
            _ => "".to_string()
        };
        let result = player_role.player_name.direct_message(&ctx, |m| m.content(message)).await;
        if let Err(_result) = result {
            msg.channel_id.say(&ctx.http, "Error!! see logs.").await?;
        }
    }
    msg.channel_id.say(&ctx.http, "done handing out role & theme.").await?;
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
    let http = Http::new(&token);
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
        Client::builder(&token, GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_PRESENCES)
            .event_handler(Handler)
            .framework(framework)
            .await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

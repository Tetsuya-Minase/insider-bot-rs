use serenity::{
    model::{channel::Message, gateway::Ready, guild::Guild, id::UserId},
    prelude::*,
};

/// get player in channel
/// # Arguments
/// * `users` - entered users
/// * `context` - &Context
/// * `message` - &Message
pub async fn get_player(users: Vec<String>, context: &Context, message: &Message) -> Vec<String> {
    let channel_members = get_channel_member(context, message).await;
    return users.iter().filter(|user| channel_members.iter().any(|member_name| &member_name == user)).map(|item| item.to_string()).collect();
}

/// get channel member
/// # Arguments
/// * `context` - &Context
/// * `message` - &Message
async fn get_channel_member(context: &Context, message: &Message) -> Vec<String> {
    let option_guild = &context.cache.guild(message.guild_id.unwrap()).await;
    if let Some(guild) = option_guild {
        let channel_members: Vec<String> = guild.members.values().map(|member| member.display_name().to_string()).collect();
        return channel_members;
    }
    vec![]
}


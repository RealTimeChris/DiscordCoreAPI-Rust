extern crate discord_core_api_rust;
use discord_core_api_rust::discord_core_client::{DiscordCoreClient, DiscordCoreClientTrait};

fn main() {
    let bot_token: String = String::new();
    let the_val: DiscordCoreClient = DiscordCoreClient::new(bot_token.clone());
    
    the_val.run_bot();

}


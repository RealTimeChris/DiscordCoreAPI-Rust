/// discord_core_client.rs - Source file for the DiscordCoreClient struct/traits etc.
/// Mar 6, 2022
/// Chris M.
/// https://github.com/RealTimeChris

#[path="foundation_entities.rs"]
mod foundation_entities;

use std::{sync::{Mutex,Arc,atomic::AtomicBool},thread,thread::{JoinHandle}};
#[path="websocket_entities.rs"]
mod websocket_entities;
use websocket_entities::{WebSocketAgent,WebSocketAgentTrait};

pub struct DiscordCoreClient {
    pub do_we_quit_locked: Arc<AtomicBool>,
    pub base_websocket: Arc<Mutex<WebSocketAgent>>,
    pub bot_token: String,
    pub the_thread:JoinHandle<bool>
}

pub trait DiscordCoreClientTrait{
    fn new(bot_token:String) -> DiscordCoreClient;
    fn run_bot(self);
}

impl DiscordCoreClientTrait for DiscordCoreClient  {

    fn new(bot_token:String) -> DiscordCoreClient{
        let do_we_quit_new: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let mut discord_core_client = DiscordCoreClient {
            do_we_quit_locked: do_we_quit_new.clone(),
            bot_token: bot_token.clone(),
            base_websocket: Arc::new(Mutex::new(WebSocketAgent::new(bot_token.clone(),String::from("gateway.discord.gg"),String::from("/?v=10&encoding=json"), String::from("443"), do_we_quit_new.clone()))),
            the_thread: thread::spawn(||->bool{return false;})};
            let new_reference = discord_core_client.base_websocket.clone();
            discord_core_client.the_thread= thread::spawn(move||->bool{loop{if !new_reference.lock().ok().unwrap().run_websocket(){break;}};return false;});
            
        return discord_core_client;
    } 

    fn run_bot(self){
        match self.the_thread.join(){
            Ok(result)=>{
                println!("join() Success: {}\n", result);
            }
            Err(error)=>{
                println!("join() Error: {:#?}\n", error);
            }
        }
    }

}

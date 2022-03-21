/// websocket_entities.rs - Source file for the WebSocket Entities struct/traits etc.
/// Mar 7, 2022
/// Chris M.
/// https://github.com/RealTimeChris
 
extern crate json;

mod erl_packer;
use std::{sync::{Arc,atomic::AtomicBool}, ops::{BitOr,Shr, Shl}};
use base64::encode;

mod ssl_clients;
use ssl_clients::{WebSocketSSLClient, SSLClientCore};

use crate::discord_core_client::websocket_entities::erl_packer::{ErlPacker, ErlPackerTrait};

pub fn generate_random_base64_encoded_auth_key()->String{
    let mut return_string=Vec::<u8>::new();
    for _x in 0..16 {
        let new_value = rand::random::<u8>();
        return_string.push(new_value);
    }
    let new_string= encode(return_string);
    return new_string;
}

#[derive(Clone, Debug, PartialEq)]
pub enum WebSocketOpCode {
    OpUnset = -01,
    OpContinuation = 0x00,
    OpText = 0x01,
    OpBinary = 0x02,
    OpClose = 0x08,
    OpPing = 0x09,
    OpPong = 0x0a
}

impl WebSocketOpCode {
    pub fn from_i8(value: i8) -> WebSocketOpCode {
        match value {
            0x00 => {
                return WebSocketOpCode::OpContinuation;
            }
            0x01 => {
                return WebSocketOpCode::OpText;
            }
            0x02 => {
                return WebSocketOpCode::OpBinary;
            }
            0x08 => {
                return WebSocketOpCode::OpClose;
            }
            0x09 => {
                return WebSocketOpCode::OpPing;
            }
            0x0a => {
                return WebSocketOpCode::OpPong;
            }
            _ => {
                return WebSocketOpCode::OpUnset;
            },
        }
    }
}
#[derive(PartialEq)]
pub enum WebSocketCurrentState {
    Connecting = 0,
    Collecting = 1,
    ParsingMessage = 4,
    ShuttingDown = 5
}

static WEB_SOCKET_PAYLOAD_LENGTH_MAGIC_LARGE: u8 = 126;
static WEB_SOCKET_PAYLOAD_LENGTH_MAGIC_HUGE: u8 = 127;
//static WEB_SOCKET_MAX_PAYLOAD_LENGTH_SMALL: u8 = 125;
static WEB_SOCKET_MASK_BIT: u8 = 1 << 7;

pub struct WebSocketAgent {
    pub current_state: WebSocketCurrentState,
    pub websocket_op_code: WebSocketOpCode,
    pub ssl_client: WebSocketSSLClient,
    pub do_we_quit: Arc<AtomicBool>,
    pub relative_path: String,
    pub bot_token: String,
    pub base_url: String
}

pub trait WebSocketAgentTrait{
    fn new(bot_token: String, base_url: String, relative_path: String, port: String, do_we_quit_new: Arc<AtomicBool>) -> WebSocketAgent;
    fn run_websocket(&mut self) -> bool;
    fn check_for_completion(&mut self);
    fn on_message_received(&mut self);
    fn parse_init_headers(&mut self);
    fn parse_message(&mut self);
    fn connect(&mut self);
}

impl WebSocketAgentTrait for WebSocketAgent{

    fn new(bot_token: String, base_url: String, relative_path: String, port: String, do_we_quit_new: Arc<AtomicBool>) -> WebSocketAgent{
        WebSocketAgent {
            ssl_client: WebSocketSSLClient::new(base_url.clone(), port,16 * 1024),
            bot_token: bot_token,
            do_we_quit: do_we_quit_new,base_url:base_url.clone(),
            current_state: WebSocketCurrentState::Connecting,
            websocket_op_code: WebSocketOpCode::OpUnset,
            relative_path: relative_path
        }
    }

    fn check_for_completion(&mut self) {
        if self.ssl_client.input_buffer.len()>0{
            self.current_state=WebSocketCurrentState::ParsingMessage;
            return;
        }
        else{
            return;
        }
    }

    fn on_message_received(&mut self){
        println!("THE RESPONSE: {}", self.ssl_client.input_buffer);
        let mut erl_packer:ErlPacker=ErlPacker::new(self.ssl_client.input_buffer.to_owned());
        println!("THE NEW RESPONSE: {}",erl_packer.buffer.buffer.as_str());
        erl_packer.parse_etf_to_json(self.ssl_client.input_buffer.clone());
        self.ssl_client.input_buffer.clear();        
        self.current_state=WebSocketCurrentState::Collecting;
    }

    fn parse_init_headers(&mut self) {
        if self.ssl_client.input_buffer.find("\r\n\r\n") != None {
            self.ssl_client.input_buffer.clear();
            self.current_state=WebSocketCurrentState::Collecting;
            return;
        }
        else{
            self.current_state=WebSocketCurrentState::Collecting;
            return;
        }
    }

    fn parse_message(&mut self)  {
        let mut new_vector = self.ssl_client.input_buffer.clone();
        let mut size_offset :usize=0;
        if new_vector.len() < 4 {
            self.current_state=WebSocketCurrentState::Collecting;
            return;
        }
        else {
            let parse_code=new_vector.remove(0) as i8;
            let parse_code_inverse=!WEB_SOCKET_MASK_BIT as i8;
            self.websocket_op_code = WebSocketOpCode::from_i8(parse_code as i8 & parse_code_inverse as i8);
            match self.websocket_op_code{                
            WebSocketOpCode::OpBinary | WebSocketOpCode::OpContinuation | WebSocketOpCode::OpPing | WebSocketOpCode::OpPong | WebSocketOpCode::OpText => {
                    let length01:i64 =new_vector.remove(0) as i64;
                    size_offset += 1;
                    let mut  payload_start_offset:i32 = 2;
                if length01 & WEB_SOCKET_MASK_BIT as i64!= 0 {
                    self.current_state=WebSocketCurrentState::Collecting;
                    return;
                }
                let mut length02 = length01;
                if length01 ==  WEB_SOCKET_PAYLOAD_LENGTH_MAGIC_LARGE as i64 {
                    if self.ssl_client.input_buffer.len() < 8 {
                        self.current_state=WebSocketCurrentState::Collecting;
                        return;
                    }
                    let length03:i64 =new_vector.remove(0) as i64;
                    size_offset += 1;
                    let length04:i64 =new_vector.remove(0) as i64;
                    size_offset += 1;
                    length02 = (length03 as i64) << 8 | length04 as i64;
                    payload_start_offset += 2;
                }
                else if length01 ==WEB_SOCKET_PAYLOAD_LENGTH_MAGIC_HUGE as i64 {
                    if self.ssl_client.input_buffer.len() < 10 {
                        self.current_state=WebSocketCurrentState::Collecting;
                        return;
                    }
                    length02 = 0;
                    let mut value=2;
                    let mut shift =56;
                    while value < 10 {
                        let length05= self.ssl_client.input_buffer.as_bytes()[value];
                        length02 |= (length05 as i64) << shift;
                        value += 1;
                        shift -= 8;
                    }
                    payload_start_offset += 8;
                }
                if self.ssl_client.input_buffer.len() < payload_start_offset as usize+ length02 as usize {
                    self.current_state=WebSocketCurrentState::Collecting;
                    return;
                }
                else {
                    let mut newer_vector: String = String::new();
                    newer_vector.reserve(length02 as usize);
                    for i in payload_start_offset as usize+size_offset  ..payload_start_offset as usize  + length02 as usize{
                        newer_vector.push(self.ssl_client.input_buffer.as_bytes()[i]as char );
                    }
                    self.ssl_client.input_buffer = newer_vector;
                    self.on_message_received();
                    let mut new_string: String = String::new();
                    for i in payload_start_offset as usize+length02 as usize -size_offset as usize..self.ssl_client.input_buffer.len(){
                        new_string.push(self.ssl_client.input_buffer.as_bytes()[i]as char);
                    }
                    self.ssl_client.input_buffer.insert_str(0, new_string.as_str());
                    return;
                }
            }
            WebSocketOpCode::OpClose => {
                println!("THE SIZE: {}", new_vector.len());
                let mut close:u16 =self.ssl_client.input_buffer.as_bytes()[0] as u16 & 0xff as u16;
				close= (close<<8) as u16;
                println!("Closing WebSocket: Code: {:#?}", close);
				close |= (self.ssl_client.input_buffer.as_bytes()[1] as u16) & 0xff as u16;
                println!("Closing WebSocket: Code: {:#?}", (self.ssl_client.input_buffer.as_bytes()[0]as u16).shl(8));
                println!("Closing WebSocket: Code: {:#?}", (self.ssl_client.input_buffer.as_bytes()[1]as u16).shl(8));
                println!("Closing WebSocket: Code: {:#?}", (self.ssl_client.input_buffer.as_bytes()[2]as u16).shl(8));
                println!("Closing WebSocket: Code: {:#?}", (self.ssl_client.input_buffer.as_bytes()[3]as u16).shl(8));
                println!("Closing WebSocket: Code: {:#?}", close);
				//close = close.bitor(self.ssl_client.input_buffer.as_bytes()[3]as u16);
                self.current_state = WebSocketCurrentState::ShuttingDown;
                return;
            }
            WebSocketOpCode::OpUnset=>{
                self.current_state=WebSocketCurrentState::Collecting;
                return;
            }
            }
        }
    }

    fn run_websocket(&mut self) -> bool {
        if self.websocket_op_code == WebSocketOpCode::OpClose {
            return false;
        }
        match self.current_state {
            WebSocketCurrentState::Connecting => {
                self.connect();
                self.ssl_client.process_io();
                self.parse_init_headers();
                return true;
            }
            WebSocketCurrentState::ParsingMessage => {
                self.parse_message();
                return true;
            }
            WebSocketCurrentState::Collecting => {
                self.ssl_client.process_io();
                self.check_for_completion();
                return true;
            }
            WebSocketCurrentState::ShuttingDown => {
                return false;
            }
        }
    }

    fn connect(&mut self){ 
        let connection_string = String::from(String::from("GET ") + self.relative_path.clone().as_str() + " HTTP/1.1\r\nHost: " + self.base_url.clone().as_str() +
            "\r\nPragma: no-cache\r\nUser-Agent: DiscordCoreAPI/1.0\r\nUpgrade: WebSocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: " + generate_random_base64_encoded_auth_key().clone().as_str() +
            "\r\nSec-WebSocket-Version: 13\r\n\r\n");
        self.ssl_client.write_data(connection_string.clone());
        self.ssl_client.process_io();
    }   
}
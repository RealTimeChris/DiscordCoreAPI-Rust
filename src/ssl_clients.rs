/// ssl_clients.rs - Source file for the "ssl clients".
/// Mar 6, 2022
/// Chris M.
/// https://github.com/RealTimeChris

extern crate openssl;
pub use std::{sync::{Mutex,Arc,atomic::AtomicBool},thread,thread::{spawn,Thread, JoinHandle},time::Duration,ops::Deref,convert::AsRef,collections::VecDeque};
pub use std::net::{TcpStream};
pub use rand::prelude::*;
pub use openssl::ssl::{Ssl, SslContext, SslStream,SslMethod, SslConnector};
pub use std::io::{Read, Write, ErrorKind};

#[derive(Default)]
pub struct HttpSSLClient {

}

impl HttpSSLClient {

}

pub struct WebSocketSSLClient {
    pub output_buffer : String,
    pub ssl_stream :SslStream<TcpStream>,
    pub input_buffer : String,
    pub max_buffer_size:i64,
    pub bytes_read : i64
}

pub trait SSLClientCore{
    fn new(base_url :String, port :String, max_buffer_size:i64) -> WebSocketSSLClient;
    fn write_data(&mut self, data: String);
    fn get_input_buffer(&self)->&String;
    fn get_bytes_read(&self) -> i64;
    fn process_io(&mut self) -> bool;
}

impl SSLClientCore for  WebSocketSSLClient {

    fn new(mut base_url :String ,mut port :String, max_buffer_size:i64)->WebSocketSSLClient {
        let mut connection_address = String::new();
        connection_address.reserve(base_url.len() + port.len());
        connection_address.push_str(base_url.as_mut_str());
        connection_address += ":";
        connection_address.push_str(port.as_mut_str());
        let mut connector = SslConnector::builder(SslMethod::tls()).unwrap();
        match connector.set_ca_file(String::from("C:/vcpkg/buildtrees/openssl/x64-windows-rel/test/certs/cacert.pem")){
            Ok(result)=>{
                println!("set_ca_file() Success: {:#?}\n", result);
            }
            Err(error)=>{
                 
                println!("set_ca_file() Error: {}\n", error);
            }

        }
        
        let stream = TcpStream::connect(connection_address.clone()).unwrap();
        match stream.set_read_timeout(Some(Duration::new(1,0))){
            Ok(result)=>{
                println!("set_read_timeout() Success: {:#?}\n", result);
            }
            Err(error)=>{
                println!("set_read_timeout() Error: {}\n", error);
            }
        }
        match stream.set_write_timeout(Some(Duration::new(1,0))){
            Ok(result)=>{
                println!("set_write_timeout() Success: {:#?}\n", result);
            }
            Err(error)=>{
                println!("set_write_timeout() Error: {}\n", error);
            }   
        }
        
        let stream_new = connector.build().connect(base_url.clone().as_str(), stream);
        let websocket_new=WebSocketSSLClient{output_buffer:String::new(),ssl_stream:stream_new.ok().take().unwrap(),input_buffer:String::new(),max_buffer_size:max_buffer_size,bytes_read:0};
        return websocket_new;
    }

    fn process_io(&mut self) -> bool{
        if self.output_buffer.len() > 0 {
            match self.ssl_stream.write(self.output_buffer.as_bytes()) {
                Ok(stream) => {
                    println!("write() Success: Size: {}\n",stream);
                    self.output_buffer.clear();
                    return true;
                }
                Err(error)=>{
                    if error.kind() == ErrorKind::TimedOut{
                        return true;
                    }
                    else{
                        println!("write() Error: {}\n", error.to_string());
                        return false;
                    }
                }
            }    
        }
        else{
            let mut data: [u8; 16*1024]= [0; 1024*16];
        
            match self.ssl_stream.read(&mut data){
                Ok(read_bytes) => {
                    let mut new_string = String::new();
                    for value in data {
                        new_string.push(value as char);
                    }
                    new_string.truncate(read_bytes);

                    self.input_buffer.push_str(&new_string.as_str());
                    println!("read() Success: Size: {}\nThe String: \n{}\n", read_bytes, new_string);
                    return true;
                }
                Err(error)=>{
                    if error.kind() == ErrorKind::TimedOut {
                        return true;
                    }
                    else{
                        println!("read() Error: {}\n", error.to_string());
                        return false;
                    }
                }
            }   
        }
    }

    fn write_data(&mut self, data: String){
        self.output_buffer.insert_str(self.output_buffer.len(),data.as_str());
        
    }

    fn get_input_buffer(&self)->&String{
        return &self.input_buffer;
    }

    fn get_bytes_read(&self) -> i64{
        return self.bytes_read;
    }
    
}


/// erl_packer.rs - Source file for the erlang-text-format parser.
/// Mar 13, 2022
/// Chris M.
/// https://github.com/RealTimeChris

extern crate json;
use json::JsonValue;
pub use num_derive::{FromPrimitive,ToPrimitive};
pub use num_traits::{FromPrimitive, ToPrimitive};
pub use std::{fmt::Debug,mem::size_of};
pub use Into;

#[derive(FromPrimitive, ToPrimitive)]
pub enum ETFTokenType{
    NewFloat = 70,
	BitBinary = 77,
	Comressed = 80,
	SmallInteger = 97,
    Integer = 98,
    Float = 99,
    Atom = 100,
    Reference = 101,
    Port = 102,
    Pid = 103,
    SmallTuple = 104,
    LargeTuple = 105,
    Nil = 106,
    String = 107,
    List = 108,
    Binary = 109,
    SmallBigInt = 110,
    LargeBigInt = 111,
    NewFunction = 112,
    Export = 113,
    NewReference = 114,
    SmallAtom = 115,
    Map = 116,
    Function = 117,
    Utf8Atom = 118,
    SmallUtf8Atom = 119
}

#[derive(Clone)]
pub struct ErlPackBuffer {

pub buffer:String,
pub offset:usize

}

pub struct ErlPacker{
    pub buffer:ErlPackBuffer
}

impl ETFTokenType {
    pub fn to_i32(&self)->i32 {
        match self {
            ETFTokenType::NewFloat =>{
                return 70;
            }
            ETFTokenType::BitBinary =>{
                return 77;
            }
            ETFTokenType::Comressed =>{
                return 80;
            }
            ETFTokenType::SmallInteger =>{return 97}
            ETFTokenType::Integer =>{return 98}
            ETFTokenType::Float =>{return 99}
            ETFTokenType::Atom =>{return 100}
            ETFTokenType::Reference =>{return 101}
            ETFTokenType::Port =>{return 102}
            ETFTokenType::Pid =>{return 103}
            ETFTokenType::SmallTuple =>{return 104}
            ETFTokenType::LargeTuple =>{return 105}
            ETFTokenType::Nil =>{return 106}
            ETFTokenType::String =>{return 107}
            ETFTokenType::List =>{return 108}
            ETFTokenType::Binary =>{return 109}
            ETFTokenType::SmallBigInt =>{return 110}
            ETFTokenType::LargeBigInt =>{return 111}
            ETFTokenType::NewFunction =>{return 112}
            ETFTokenType::Export =>{return 113}
            ETFTokenType::NewReference =>{return 114}
            ETFTokenType::SmallAtom =>{return 115}
            ETFTokenType::Map =>{return 116}
            ETFTokenType::Function =>{return 117}
            ETFTokenType::Utf8Atom =>{return 118}
            ETFTokenType::SmallUtf8Atom =>{return 119}
        }
    }
}

impl ErlPacker{
    pub fn new(buffer:String)->ErlPacker{
        return ErlPacker{buffer:ErlPackBuffer{offset:0, buffer:buffer}};
    }
}

pub trait ErlPackerTrait{
    fn parse_etf_to_json(&mut self ,data_to_parse: String ) ->JsonValue;
    
	fn read_bits<T:Sized+std::ops::BitOrAssign+Copy+Into<T>+From<T>+FromPrimitive+ToPrimitive+Debug>(&mut self,buffer:ErlPackBuffer, return_type:&mut T);
    fn etf_byte_order<T:FromPrimitive+ToPrimitive>(new_value:T, newer_value:&mut T);
    fn single_value_etf_to_json(&mut self, buffer:ErlPackBuffer)->JsonValue;
}

impl ErlPackerTrait for ErlPacker {
    

    fn parse_etf_to_json(&mut self ,data_to_parse: String ) ->JsonValue{
        let buffer:ErlPackBuffer=ErlPackBuffer { buffer:data_to_parse, offset:0 };
        let mut version:u8=0;
        self.read_bits(buffer.clone(), &mut version );
        println!("THE BUFFER: {}",buffer.buffer);
        return self.single_value_etf_to_json(buffer);
		
	}
    
	fn read_bits<T:Sized+std::ops::BitOrAssign+Copy+Into<T>+From<T>+FromPrimitive+ToPrimitive+Debug>(&mut self,mut buffer:ErlPackBuffer, return_type:&mut T){
		const BYTE_SIZE:usize=8;
        println!("THE SIZE: {}",buffer.buffer.len());
        println!("THE SIZE: {}",size_of::<T>());
		if buffer.offset + size_of::<T>() as usize> buffer.buffer.len(){
			panic!("ETF Parse Error: read_bits() past end of buffer");
		}
        if size_of::<T>() ==1 {
             let mut newer_value:u8=return_type.to_u8().unwrap();
             for i in 0..size_of::<T>() {
                newer_value |= (buffer.buffer.as_bytes()[buffer.offset + i])<< (i * BYTE_SIZE) as u8;
            }
            buffer.offset += size_of::<T>();
            let newest_value=T::from_u8(newer_value).unwrap();
		    println!("THE NEWER VALUE: {:#?}",newest_value);
		    ErlPacker::etf_byte_order::<T>(newest_value, return_type);
            println!("THE NEWER VALUE: {:#?}", return_type);
        }
        if size_of::<T>() ==2 {
             let mut newer_value:u16=return_type.to_u16().unwrap();
             for i in 0..size_of::<T>() {
                newer_value |= ((buffer.buffer.as_bytes()[buffer.offset + i] as u16)<< (i * BYTE_SIZE)) as u16;
            }
            buffer.offset += size_of::<T>();
            let newest_value=T::from_u16(newer_value).unwrap();
		    ErlPacker::etf_byte_order::<T>(newest_value, return_type);
        }
        if size_of::<T>() ==4 {
            let mut newer_value:u32=return_type.to_u32().unwrap();
            for i in 0..size_of::<T>() {
                newer_value |= ((buffer.buffer.as_bytes()[buffer.offset + i] as u32)<< (i * BYTE_SIZE)) as u32;
            }
            buffer.offset += size_of::<T>();
		    let newest_value=T::from_u32(newer_value).unwrap();
            println!("THE NEWER VALUE: {:#?}",newest_value);
		    ErlPacker::etf_byte_order::<T>(newest_value, return_type);
            println!("THE NEWER VALUE: {:#?}", return_type);
        }
        if size_of::<T>() ==8 {
            let mut newer_value:u64= return_type.to_u64().unwrap();
            for i in 0..size_of::<T>() {
                newer_value |= ((buffer.buffer.as_bytes()[buffer.offset + i] as u64)<< (i * BYTE_SIZE))as u64;
            }
            buffer.offset += size_of::<T>();
		    let newest_value=T::from_u64(newer_value).unwrap();
		    ErlPacker::etf_byte_order::<T>(newest_value, return_type);
        }
	}

    fn etf_byte_order<T:FromPrimitive+ToPrimitive>(new_value:T, newer_value:&mut T){
        const BYTE_SIZE: u8 = 8;
        if size_of::<T>() == 1 {
            let  mut _the_value:u8=0;
            for i in 0.. size_of::<T>() {
                _the_value = (new_value.to_u8().unwrap() >> (BYTE_SIZE * i as u8)as u16) << BYTE_SIZE *(size_of::<T>() - i - 1) as u8;
                return;
            }
            *newer_value= T::from_u8(_the_value).unwrap();
        }
        if size_of::<T>() == 2 {
            let  mut _the_value:u16=0;
            for i in 0.. size_of::<T>() {
                _the_value = (new_value.to_u16().unwrap() >> (BYTE_SIZE * i as u8)as u16) << BYTE_SIZE *(size_of::<T>() - i - 1) as u8;
                return;
            }
            *newer_value= T::from_u16(_the_value).unwrap();
        }
        if size_of::<T>() == 4 {
            let  mut _the_value:u32=0;
            for i in 0.. size_of::<T>() {
                _the_value = (new_value.to_u32().unwrap() >> (BYTE_SIZE * i as u8)as u32) << BYTE_SIZE *(size_of::<T>() - i - 1) as u8;
                return;
            }
            *newer_value= T::from_u32(_the_value).unwrap();
        }
        if size_of::<T>() == 8 {
            let  mut _the_value:u64=0;
            for i in 0.. size_of::<T>() {
                _the_value = (new_value.to_u64().unwrap() >> (BYTE_SIZE * i as u8)as u16) << BYTE_SIZE *(size_of::<T>() - i - 1) as u8;
                return;
            }
            *newer_value= T::from_u64(_the_value).unwrap();
        }
    }

    fn single_value_etf_to_json(&mut self, buffer:ErlPackBuffer)->JsonValue{
		if buffer.offset >= buffer.buffer.len() as usize{
			panic!("ETF Parse Error: Read past end of ETF buffer");
		}
		let the_type:ETFTokenType=ETFTokenType::Atom;
		self.read_bits(buffer,&mut the_type.to_i32());
		match the_type {
		ETFTokenType::SmallInteger=> {
			//return ErlPacker::parseSmallInteger(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::Integer=> {
			//return ErlPacker::parseInteger(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::Float=> {
			//return ErlPacker::parseFloat(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::NewFloat=> {
			//return ErlPacker::parseNewFloat(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::Atom=> {
			//return ErlPacker::parseAtom(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::SmallAtom=> {
			//return ErlPacker::parseSmallAtom(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::SmallTuple=> {
			//return ErlPacker::parseSmallTuple(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::LargeTuple=> {
			//return ErlPacker::parseLargeTuple(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::Nil=> {
			//return ErlPacker::parseNil();
            return json::JsonValue::new_object();
		}
		ETFTokenType::String=> {
			//return ErlPacker::parseStringAsList(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::List=> {
			//return ErlPacker::parseList(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::Map=> {
			//return ErlPacker::parseMap(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::Binary=> {
			//return ErlPacker::parseBinary(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::SmallBigInt=> {
			//return ErlPacker::parseSmallBigint(buffer);
            return json::JsonValue::new_object();
		}
		ETFTokenType::LargeBigInt=> {
			//return ErlPacker::parseLargeBigint(buffer);
            return json::JsonValue::new_object();
		}
		_=> {
			panic!("ETF Parse Error: Unknown data type in ETF");
		}
		}
	}
}

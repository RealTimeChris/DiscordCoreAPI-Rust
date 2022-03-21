/// foundation_entities.rs - Source file for the Foundation Entities structs/traits etc.
/// Mar 7, 2022
/// Chris M.
/// https://github.com/RealTimeChris
pub use std::{sync::{Mutex,Arc},collections::VecDeque};

#[derive(Default)]
pub struct UnboundedMessageBlock<T> {
     ts_accessor:Arc<Mutex<VecDeque<T>>>
}

pub trait UnboundedMessageBlocktTrait<T> {
     fn try_receive(&mut self, arg: &mut T) -> bool;
     fn send(&mut self, arg: &T);
}

impl<T: Clone> UnboundedMessageBlocktTrait<T> for UnboundedMessageBlock<T> {     
     fn try_receive(&mut self, arg: &mut T) -> bool {
          let ts_accessor_new= self.ts_accessor.clone();
          let mut arg_new = ts_accessor_new.lock().unwrap();
          if arg_new.len() > 0 {
               *arg = arg_new.pop_back().unwrap();
               return true;
          } else {
               return false;
          }
     }

     fn send(self: &mut Self, arg: &T) {
          let ts_accessor_new= self.ts_accessor.clone();
          ts_accessor_new.lock().unwrap().push_front(arg.clone());
     }
}
 
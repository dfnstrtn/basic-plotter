extern crate serde;
use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize,Debug)]
pub struct DataFormat{
    pub name:String,
    pub color:[u8;3],
    pub points:[f32;2]
}

extern crate serialize = "serialize#0.10-pre-alpha";

use serialize::json::{from_str, Json, Decoder};
use serialize::Decodable;
use std::any::Any;
use std::intrinsics::TypeId;

//#[deriving(Decodable)]
struct OptTest{
    req: ~str,
    opt: Option<uint>,
}

fn main(){
    let tmp = from_str("{\"req\": \"foo\"}");
    //println!("{:?}", tmp.unwrap().to_str());
    //let mut decoder = Decoder::new(tmp.unwrap());
    //let tmp: OptTest = Decodable::decode(&mut decoder);
    //println!("{:?}", tmp);

    //println!("{:?}", TypeId::of::<Option<int>>());
    //println!("{:?}", TypeId::of::<Option<uint>>());
    
    //let tmp = from_str("{\"opt\": 50}");
    //let mut decoder = Decoder::new(tmp.unwrap());
    //let tmp: OptTest = Decodable::decode(&mut decoder);
    //println!("{:?}", tmp);
}

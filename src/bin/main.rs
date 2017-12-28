extern crate clojit_vm;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::env;

use std::fs::File;
use std::io::prelude::*;

use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
struct CljType {
    #[serde(rename = "type-id")]
    typeid: u64,
    size: u64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Inst {
    ABC {
        op : String,
        a : u8,
        b : u8,
        c : u8
    },
    AD {
        op : String,
        a : u8,
        d : Option<u16>
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CsonInput {
    #[serde(rename = "CFLOAT")]
    cfloat : Vec<f64>,
    #[serde(rename = "CINT")]
    cint: Vec<u64>,
    #[serde(rename = "CSTR")]
    cstr: Vec<String>,
    #[serde(rename = "CKEY")]
    ckey: Vec<String>,
    vtable: BTreeMap<u64, BTreeMap<u64, u64>>,
    types: Vec<CljType>,
    bytecode : Vec<Inst>
}

fn main() {
    println!("Start clojit-vm");

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let parsed_cson: CsonInput = serde_json::from_str(&contents).unwrap();


    println!("Parsed Bytecode: ");

    println!("{:?}", parsed_cson);

    println!("Result: ");



}
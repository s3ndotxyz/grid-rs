#![feature(random)]
use serde::Serialize;
use std::random::random;

#[derive(Serialize)]
struct MyOutput {
    env: String,
    uuid: String,
}

#[grid_rs::main]
fn main(_input: &[u8]) -> Result<Vec<u8>, String> {
    let bits: u128 = random();
    let g1 = (bits >> 96) as u32;
    let g2 = (bits >> 80) as u16;
    let g3 = (0x4000 | (bits >> 64) & 0x0fff) as u16;
    let g4 = (0x8000 | (bits >> 48) & 0x3fff) as u16;
    let g5 = (bits & 0xffffffffffff) as u64;
    let uuid = format!("{g1:08x}-{g2:04x}-{g3:04x}-{g4:04x}-{g5:012x}");

    let output = MyOutput {
        env: std::env::var("env").unwrap().to_string(),
        uuid,
    };

    Ok(serde_json::to_vec(&output).unwrap())
}

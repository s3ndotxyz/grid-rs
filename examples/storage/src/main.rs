use grid_rs::{kvs::Storage};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct MyInput {
    username: String,
    data: Vec<u8>,
}

#[derive(Serialize)]
struct MyOutput {
    status: String,
    result: Vec<u8>,
}

#[grid_rs::main]
fn main(input: &[u8]) -> Result<Vec<u8>, String> {
    let input: MyInput = serde_json::from_slice(input).unwrap();
    
    Storage::set(&input.username, &input.data);
    
    Storage::put(&input.username, &input.data);
    let output = MyOutput {
        status: "success".to_string(),
        result: input.data,
    };
    
    Ok(serde_json::to_vec(&output).unwrap())
}

use grid_rs::kvs::Storage;
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
    let input: MyInput = match serde_json::from_slice(input) {
        Ok(input) => input,
        Err(e) => {
            return Err(format!(
                "JSON deserialization failed: {e}. Input was: {}",
                String::from_utf8_lossy(input)
            ));
        }
    };

    Storage::put(format!("user-{}", input.username).as_str(), &input.username, input.data.as_slice());

    let output = MyOutput {
        status: "success".to_string(),
        result: input.data,
    };

    Ok(serde_json::to_vec(&output).unwrap())
}

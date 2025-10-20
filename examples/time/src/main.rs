use serde::Serialize;

#[derive(Serialize)]
struct MyOutput {
    time: u64,
}

#[grid_rs::main]
fn main(_input: &[u8]) -> Result<Vec<u8>, String> {
    let output = MyOutput {
        time: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::ZERO)
            .as_secs(),
    };

    Ok(serde_json::to_vec(&output).unwrap())
}

# Slipstream Rust SDK (Grid)

## Overview

This is a library for using host functions from the [*Slipstream runtime*](https://github.com/s3ndotxyz/runtime). It's more or less a wrapper for linked Wasm imports.

## Features

* *kvs*. Provides access to the native in-runtime key-value store. You should be able to manage keys, values, and stores from within your functions. *Note:* stores are based on namespace set within a function. This allows for multiple stores per function, persisting instance state.
* *ws*. Provides server and client connections for modules.

We have plans to enable a few other things such as message queues in the near future. Stay tuned!

## Example

Here's a simple example of a function that stores a username and a vector of bytes in the key-value store:

```rust
use grid_rs::{kvs::Storage};
use serde::{Deserialize, Serialize};

#[grid_rs::main]
fn main(input: &[u8]) -> Result<Vec<u8>, String> {
    let input: MyInput = match serde_json::from_slice(input) {
        Ok(input) => input,
        Err(e) => {
            return Err(format!(
                "JSON deserialization failed: {e}. Input was: {}",
                String::from_utf8_lossy(input)));
        }
    };

    Storage::put(format!("user-{}", input.username).as_str(), &input.username, input.data.as_slice());

    let output = MyOutput {
        status: "success".to_string(),
        result: input.data,
    };

    Ok(serde_json::to_vec(&output).unwrap())
}

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
```

Find more examples [here](https://github.com/s3ndotxyz/grid-rs/tree/main/examples).

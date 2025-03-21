use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}

#[derive(Serialize, Deserialize)]
struct Recipe {}

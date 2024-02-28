//! A simple program to be proven inside the zkVM.
#![no_main]

use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;
use std::collections::HashMap;

sp1_zkvm::entrypoint!(main);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub text: String,
    pub timestamp: u64,
}

type Checkpoints = HashMap<u64, (Vec<u8>, Vec<ChatMessage>)>;

pub fn main() {
    let checkpoints = sp1_zkvm::io::read::<Checkpoints>();
    let search_string = sp1_zkvm::io::read::<String>();

    let mut result: Option<(ChatMessage, u64)> = None;

    for (checkpoint_timestamp, (hash, messages)) in checkpoints.into_iter() {
        let hasher = Sha256::new();

        assert!(hasher.finalize().to_vec() == hash);

        for message in messages {
            if message.text.contains(&search_string) {
                result = Some((message, checkpoint_timestamp));
                break;
            }
        }
    }

    sp1_zkvm::io::write(&true);
}

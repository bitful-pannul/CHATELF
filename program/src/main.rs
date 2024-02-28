//! A simple program to be proven inside the zkVM.
#![no_main]

use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;
use std::collections::HashMap;

sp1_zkvm::entrypoint!(main);

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatMessage {
    pub sender: String,
    pub text: String,
    pub timestamp: u64,
}

/// list of results: checkpoint_timestamp, sender, text
type ProofResult = Vec<(u64, String, String)>;

type Checkpoints = HashMap<u64, (Vec<u8>, Vec<ChatMessage>)>;

pub fn main() {
    let checkpoints = sp1_zkvm::io::read::<Checkpoints>();
    let search_string = sp1_zkvm::io::read::<String>();

    let mut result: ProofResult = Vec::new();

    for (checkpoint_timestamp, (hash, messages)) in checkpoints.into_iter() {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_vec(&messages).unwrap());
        assert!(hasher.finalize().to_vec() == hash);

        for message in messages {
            if message.text.contains(&search_string) {
                result.push((checkpoint_timestamp, message.sender, message.text));
            }
        }
    }

    sp1_zkvm::io::write(&result);
}

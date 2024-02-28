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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum ProofResult {
    NotFound,
    Found {
        checkpoint_timestamp: u64,
        sender: String,
        text: String,
    },
}

type Checkpoints = HashMap<u64, (Vec<u8>, Vec<ChatMessage>)>;

pub fn main() {
    let checkpoints = sp1_zkvm::io::read::<Checkpoints>();
    let search_string = sp1_zkvm::io::read::<String>();

    let mut result = ProofResult::NotFound;

    for (checkpoint_timestamp, (hash, messages)) in checkpoints.into_iter() {
        // let mut hasher = Sha256::new();
        // hasher.update(serde_json::to_vec(&messages).unwrap());
        // assert!(hasher.finalize().to_vec() == hash);

        for message in messages {
            if message.text.contains(&search_string) {
                result = ProofResult::Found {
                    checkpoint_timestamp,
                    sender: message.sender,
                    text: message.text,
                };
                break;
            }
        }
        if result != ProofResult::NotFound {
            break;
        }
    }

    sp1_zkvm::io::write(&result);
}

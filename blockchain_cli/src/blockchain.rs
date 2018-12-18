extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    previous_hash: String,
    merkle_hash: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    difficulty: u32,
    miner_address: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
            difficulty,
            miner_address,
            reward: 100.0,
        };

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.current_transactions.push(Transaction {
            sender,
            receiver,
            amount,
        });

        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            previous_hash: self.last_hash(),
            merkle_hash: String::new(),
            difficulty: self.difficulty,
        };

        let reward_transaction = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_transaction);
        block.transactions.append(&mut self.current_transactions);
        block.count = block.transactions.len() as u32;
        block.header.merkle_hash = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(current_transactions: Vec<Transaction>) -> String {
        let mut merkle_hash = Vec::new();

        for transaction in &current_transactions {
            let hash = Chain::hash(transaction);
            merkle_hash.push(hash);
        }

        // if odd...
        if merkle_hash.len() % 2 == 1 {
            let last = merkle_hash.last().cloned().unwrap();
            merkle_hash.push(last);
        }

        while merkle_hash.len() > 1 {
            let mut hash_1 = merkle_hash.remove(0);
            let hash_2 = merkle_hash.remove(0);
            hash_1.push_str(&hash_2);
            let new_hash = Chain::hash(&hash_1);
            merkle_hash.push(new_hash);
        }
        merkle_hash.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let result = hasher.result();
        let vec_result = result.to_vec();

        Chain::hex_to_string(vec_result.as_slice())
    }

    pub fn hex_to_string(vec_result: &[u8]) -> String {
        let mut s = String::new();
        for byte in vec_result {
            write!(&mut s, "{:x}", byte).expect("unable to write")
        }
        s
    }
}

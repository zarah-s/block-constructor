use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::process;

#[derive(Debug, Clone, PartialEq)]
struct Pool {
    tx_id: String,
    fee: i32,
    weight: i32,
    parent_txids: Option<Vec<String>>,
}

impl Pool {
    pub fn new(tx_id: String, fee: i32, weight: i32, parent_txids: Option<Vec<String>>) -> Self {
        Self {
            tx_id,
            fee,
            weight,
            parent_txids,
        }
    }
}

fn main() {
    const MEMPOOL_FILE_PATH: &str = "./mempool.csv";
    const BLOCK_SAMPLE_PATH: &str = "./block_sample.txt";

    let mut pool_transactions: Vec<Pool> = Vec::new();

    let file_contents = fs::read_to_string(MEMPOOL_FILE_PATH).unwrap();

    for line in file_contents.lines() {
        let params: Vec<&str> = line.split(",").filter(|pred| !pred.is_empty()).collect();
        if params.len() > 3 {
            let mut stringified_parent_ids = Vec::new();
            let parent_ids: Vec<&str> = params[3]
                .split(";")
                .filter(|pred| !pred.is_empty())
                .collect();

            for stringified in parent_ids {
                stringified_parent_ids.push(stringified.to_string());
            }

            let structured = Pool::new(
                params[0].to_string(),
                params[1].parse().unwrap(),
                params[2].parse().unwrap(),
                Some(stringified_parent_ids),
            );
            pool_transactions.push(structured);
        } else {
            let structured = Pool::new(
                params[0].to_string(),
                params[1].parse().unwrap(),
                params[2].parse().unwrap(),
                None,
            );
            pool_transactions.push(structured);
        }
    }

    let mut arranged_transactions = arrange_transactions(pool_transactions);
    remove_duplicate_transactions(&mut arranged_transactions);

    let mut block_sample = OpenOptions::new()
        .write(true)
        .append(true)
        .open(BLOCK_SAMPLE_PATH)
        .unwrap();

    for transaction in arranged_transactions {
        if let Err(e) = writeln!(block_sample, "{}", transaction.tx_id) {
            eprintln!("Couldn't write to file {}", e);
            process::exit(1);
        }
    }
}

fn arrange_transactions(transactions: Vec<Pool>) -> Vec<Pool> {
    let mut arranged_transactions = Vec::new();

    for transaction in transactions.clone() {
        if let Some(_parent_ids) = transaction.parent_txids {
            for parent_tx in _parent_ids {
                let find_parent = transactions.iter().find(|pred| pred.tx_id == parent_tx);
                if let Some(_dd) = find_parent {
                    arranged_transactions.push(_dd.clone());
                }
            }
        } else {
            arranged_transactions.push(transaction);
        }
    }

    arranged_transactions
}

fn remove_duplicate_transactions(transactions: &mut Vec<Pool>) {
    transactions.dedup_by(|a, b| a.tx_id == b.tx_id);
}

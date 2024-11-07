extern crate log;

use fern::Dispatch;
use chrono::Local;
use log::{info, error, LevelFilter};
use thiserror::Error;
use std::fmt;

// Make the terminal module public and available
pub mod terminal;

#[derive(Debug)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "Deposit"),
            TransactionType::Withdrawal => write!(f, "Withdrawal"),
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub wallet_address: String,
    pub amount: i64,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} of {} to {}",
            self.transaction_type, self.amount, self.wallet_address
        )
    }
}

// Custom error type
#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Invalid transaction amount: {0}")]
    InvalidAmount(i64),
    #[error("Insufficient funds for withdrawal of {requested}. Available balance: {available}")]
    InsufficientFunds {
        requested: i64,
        available: i64,
    },
}

pub fn init_logging(test_name: &str) {
    let test_name = test_name.to_string();
    let log_file_path = format!(
        "/home/jtdev/Desktop/web_3/ryz-labs/logs/tests/{}_log_output.log",
        test_name
    );

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                test_name,
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(fern::log_file(log_file_path).unwrap())
        .apply()
        .unwrap();
}

pub fn log_section_header(section: &str) {
    info!("========== {} ==========", section);
}

pub fn calculate_wallet_balance(
    transactions: &[Transaction],
    wallet_address: &str,
) -> Result<i64, WalletError> {
    use TransactionType::*;

    let mut balance = 0;

    for tx in transactions.iter().filter(|tx| tx.wallet_address == wallet_address) {
        if tx.amount < 0 {
            error!(
                "Invalid transaction amount: {} in transaction {:?}",
                tx.amount, tx
            );
            return Err(WalletError::InvalidAmount(tx.amount));
        }

        match tx.transaction_type {
            Deposit => {
                info!("Deposit of {} to {}", tx.amount, tx.wallet_address);
                balance += tx.amount;
            }
            Withdrawal => {
                if balance < tx.amount {
                    error!(
                        "Insufficient funds for withdrawal of {} from {}. Available balance: {}",
                        tx.amount, tx.wallet_address, balance
                    );
                    return Err(WalletError::InsufficientFunds {
                        requested: tx.amount,
                        available: balance,
                    });
                }
                info!("Withdrawal of {} from {}", tx.amount, tx.wallet_address);
                balance -= tx.amount;
            }
        }
    }

    info!("Final balance for wallet {}: {}", wallet_address, balance);
    Ok(balance)
}

/// Prints the transaction history for a given wallet, showing the transaction type, amount, and running balance.
pub fn print_transaction_history(transactions: &[Transaction], wallet_address: &str) {
    let mut balance = 0;
    println!("Transaction history for wallet {}:", wallet_address);
    for tx in transactions.iter().filter(|tx| tx.wallet_address == wallet_address) {
        match tx.transaction_type {
            TransactionType::Deposit => balance += tx.amount,
            TransactionType::Withdrawal => balance -= tx.amount,
        }
        println!("{} | Running balance: {}", tx, balance);
    }
} 
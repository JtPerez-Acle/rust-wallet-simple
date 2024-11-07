//! Core library for the Ryz Labs Wallet Balance Tracker
//! Provides transaction management, balance calculation, and logging functionality

extern crate log;

use fern::Dispatch;
use chrono::Local;
use log::{info, error, LevelFilter};
use thiserror::Error;
use std::fmt;

// Export terminal module for external use
pub mod terminal;

/// Represents the types of transactions supported by the wallet system
#[derive(Debug)]
pub enum TransactionType {
    /// Represents funds being added to a wallet
    Deposit,
    /// Represents funds being removed from a wallet
    Withdrawal,
}

// Implement display formatting for transaction types
impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "Deposit"),
            TransactionType::Withdrawal => write!(f, "Withdrawal"),
        }
    }
}

/// Represents a single transaction in the wallet system
#[derive(Debug)]
pub struct Transaction {
    /// Type of transaction (Deposit/Withdrawal)
    pub transaction_type: TransactionType,
    /// Address of the wallet involved in the transaction
    pub wallet_address: String,
    /// Amount of funds involved in the transaction
    pub amount: i64,
}

// Implement display formatting for transactions
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} of {} to {}",
            self.transaction_type, self.amount, self.wallet_address
        )
    }
}

/// Custom error types for wallet operations
#[derive(Debug, Error)]
pub enum WalletError {
    /// Error for invalid transaction amounts (e.g., negative values)
    #[error("Invalid transaction amount: {0}")]
    InvalidAmount(i64),
    /// Error for insufficient funds during withdrawal
    #[error("Insufficient funds for withdrawal of {requested}. Available balance: {available}")]
    InsufficientFunds {
        requested: i64,
        available: i64,
    },
}

/// Initializes the logging system with test-specific configuration
/// 
/// # Arguments
/// * `test_name` - Identifier for the test being executed
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

/// Logs a section header for test organization
/// 
/// # Arguments
/// * `section` - Name of the test section
pub fn log_section_header(section: &str) {
    info!("========== {} ==========", section);
}

/// Calculates the current balance for a specific wallet
/// 
/// # Arguments
/// * `transactions` - Slice of transactions to process
/// * `wallet_address` - Address of the wallet to calculate balance for
/// 
/// # Returns
/// * `Result<i64, WalletError>` - Calculated balance or error if validation fails
pub fn calculate_wallet_balance(
    transactions: &[Transaction],
    wallet_address: &str,
) -> Result<i64, WalletError> {
    use TransactionType::*;

    let mut balance = 0;

    // Process each transaction for the specified wallet
    for tx in transactions.iter().filter(|tx| tx.wallet_address == wallet_address) {
        // Validate transaction amount
        if tx.amount < 0 {
            error!(
                "Invalid transaction amount: {} in transaction {:?}",
                tx.amount, tx
            );
            return Err(WalletError::InvalidAmount(tx.amount));
        }

        // Update balance based on transaction type
        match tx.transaction_type {
            Deposit => {
                info!("Deposit of {} to {}", tx.amount, tx.wallet_address);
                balance += tx.amount;
            }
            Withdrawal => {
                // Verify sufficient funds for withdrawal
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

/// Displays transaction history for a specific wallet
/// 
/// # Arguments
/// * `transactions` - Slice of transactions to display
/// * `wallet_address` - Address of the wallet to show history for
pub fn print_transaction_history(transactions: &[Transaction], wallet_address: &str) {
    let mut balance = 0;
    println!("Transaction history for wallet {}:", wallet_address);
    
    // Display each transaction with running balance
    for tx in transactions.iter().filter(|tx| tx.wallet_address == wallet_address) {
        match tx.transaction_type {
            TransactionType::Deposit => balance += tx.amount,
            TransactionType::Withdrawal => balance -= tx.amount,
        }
        println!("{} | Running balance: {}", tx, balance);
    }
} 
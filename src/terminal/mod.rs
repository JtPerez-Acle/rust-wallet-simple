use std::io::{self, Write};
use crate::{Transaction, TransactionType, print_transaction_history, calculate_wallet_balance};
use log::{info, error};
use chrono::Local;
use std::fs;
use std::path::Path;

pub struct WalletTerminal {
    pub(crate) transactions: Vec<Transaction>,
}

impl WalletTerminal {
    pub fn new() -> Self {
        if let Err(e) = Self::init_logging() {
            eprintln!("Warning: Failed to initialize logging: {}", e);
        }
        info!("Initializing new WalletTerminal instance");
        WalletTerminal {
            transactions: Vec::new(),
        }
    }

    fn init_logging() -> io::Result<()> {
        // Create logs directory structure
        let log_dir = Path::new("logs/src");
        fs::create_dir_all(log_dir)?;

        let log_file_path = log_dir.join(format!(
            "terminal_{}.log",
            Local::now().format("%Y%m%d_%H%M%S")
        ));

        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{} [{}] [Terminal] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    message
                ))
            })
            .level(log::LevelFilter::Info)
            .chain(fern::log_file(log_file_path)?)
            .apply()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(())
    }

    pub fn run(&mut self) {
        info!("Starting wallet terminal session");
        println!("Welcome to Ryz Labs Wallet Terminal!");
        
        loop {
            match self.show_menu() {
                Ok(should_exit) => {
                    if should_exit {
                        info!("Terminating wallet terminal session");
                        println!("Thank you for using Ryz Labs Wallet Terminal!");
                        break;
                    }
                }
                Err(e) => {
                    error!("Menu error: {}", e);
                    println!("Error: {}", e);
                }
            }
        }
    }

    fn show_menu(&mut self) -> io::Result<bool> {
        println!("\nPlease select an option:");
        println!("1. Check Balance");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("4. View Transaction History");
        println!("5. Exit");
        print!("\nEnter your choice (1-5): ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                info!("Selected: Check Balance");
                self.check_balance()?;
            }
            "2" => {
                info!("Selected: Deposit");
                self.deposit()?;
            }
            "3" => {
                info!("Selected: Withdraw");
                self.withdraw()?;
            }
            "4" => {
                info!("Selected: View History");
                self.view_history()?;
            }
            "5" => {
                info!("Selected: Exit");
                return Ok(true);
            }
            _ => {
                error!("Invalid menu choice entered: {}", choice.trim());
                println!("Invalid choice. Please try again.");
            }
        }

        Ok(false)
    }

    fn get_wallet_address(&self) -> io::Result<String> {
        print!("Enter wallet address: ");
        io::stdout().flush()?;
        let mut address = String::new();
        io::stdin().read_line(&mut address)?;
        let address = address.trim().to_string();
        info!("Wallet address entered: {}", address);
        Ok(address)
    }

    fn get_amount(&self) -> io::Result<i64> {
        print!("Enter amount: ");
        io::stdout().flush()?;
        let mut amount_str = String::new();
        io::stdin().read_line(&mut amount_str)?;
        match amount_str.trim().parse() {
            Ok(amount) => {
                info!("Amount entered: {}", amount);
                Ok(amount)
            }
            Err(_) => {
                error!("Invalid amount entered: {}", amount_str.trim());
                println!("Invalid amount. Please enter a valid number.");
                Ok(0)
            }
        }
    }

    fn check_balance(&self) -> io::Result<()> {
        let wallet_address = self.get_wallet_address()?;
        match calculate_wallet_balance(&self.transactions, &wallet_address) {
            Ok(balance) => {
                info!("Balance check successful for {}: {}", wallet_address, balance);
                println!("Balance for wallet {}: {}", wallet_address, balance);
            }
            Err(e) => {
                error!("Balance check failed for {}: {}", wallet_address, e);
                println!("Error checking balance: {}", e);
            }
        }
        Ok(())
    }

    fn deposit(&mut self) -> io::Result<()> {
        let wallet_address = self.get_wallet_address()?;
        let amount = self.get_amount()?;
        
        if amount <= 0 {
            error!("Invalid deposit amount attempted: {}", amount);
            println!("Amount must be positive");
            return Ok(());
        }

        self.transactions.push(Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: wallet_address.clone(),
            amount,
        });
        info!("Successful deposit of {} to wallet {}", amount, wallet_address);
        println!("Successfully deposited {} to the wallet", amount);
        Ok(())
    }

    fn withdraw(&mut self) -> io::Result<()> {
        let wallet_address = self.get_wallet_address()?;
        let amount = self.get_amount()?;

        if amount <= 0 {
            error!("Invalid withdrawal amount attempted: {}", amount);
            println!("Amount must be positive");
            return Ok(());
        }

        match calculate_wallet_balance(&self.transactions, &wallet_address) {
            Ok(balance) if balance >= amount => {
                self.transactions.push(Transaction {
                    transaction_type: TransactionType::Withdrawal,
                    wallet_address: wallet_address.clone(),
                    amount,
                });
                info!("Successful withdrawal of {} from wallet {}", amount, wallet_address);
                println!("Successfully withdrew {} from the wallet", amount);
            }
            Ok(balance) => {
                error!("Insufficient funds for withdrawal: requested {}, available {}", amount, balance);
                println!("Insufficient funds. Available balance: {}", balance);
            }
            Err(e) => {
                error!("Withdrawal error for wallet {}: {}", wallet_address, e);
                println!("Error: {}", e);
            }
        }
        Ok(())
    }

    fn view_history(&self) -> io::Result<()> {
        let wallet_address = self.get_wallet_address()?;
        info!("Viewing transaction history for wallet {}", wallet_address);
        print_transaction_history(&self.transactions, &wallet_address);
        Ok(())
    }
} 
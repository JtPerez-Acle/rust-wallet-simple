use ryz_labs::*;
use std::sync::Once;

// Ensures logging initialization occurs only once across all test executions
static INIT: Once = Once::new();

// Initializes logging system with test-specific configuration
fn initialize(test_name: &str) {
    INIT.call_once(|| {
        init_logging(test_name);
    });
}

#[test]
fn test_transaction_creation() {
    // Initialize test environment with unique identifier
    initialize("test_transaction_creation");
    log_section_header("Start Test: Transaction Creation");

    // Create sample deposit transaction for testing
    let deposit_transaction = Transaction {
        transaction_type: TransactionType::Deposit,
        wallet_address: String::from("wallet_1"),
        amount: 100,
    };

    // Create sample withdrawal transaction for testing
    let withdrawal_transaction = Transaction {
        transaction_type: TransactionType::Withdrawal,
        wallet_address: String::from("wallet_2"),
        amount: 50,
    };

    // Verify deposit transaction properties
    match deposit_transaction.transaction_type {
        TransactionType::Deposit => (),
        _ => panic!("Expected Deposit transaction type"),
    }
    assert_eq!(deposit_transaction.wallet_address, "wallet_1");
    assert_eq!(deposit_transaction.amount, 100);

    // Verify withdrawal transaction properties
    match withdrawal_transaction.transaction_type {
        TransactionType::Withdrawal => (),
        _ => panic!("Expected Withdrawal transaction type"),
    }
    assert_eq!(withdrawal_transaction.wallet_address, "wallet_2");
    assert_eq!(withdrawal_transaction.amount, 50);

    log_section_header("End Test: Transaction Creation");
}

#[test]
fn test_calculate_wallet_balance() {
    // Test basic balance calculation functionality
    initialize("test_calculate_wallet_balance");
    log_section_header("Start Test: Calculate Wallet Balance");

    // Create test transactions for balance calculation
    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: String::from("wallet_1"),
            amount: 100,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: String::from("wallet_1"),
            amount: 30,
        },
    ];

    // Verify correct balance calculation
    let balance = calculate_wallet_balance(&transactions, "wallet_1").unwrap();
    assert_eq!(balance, 70);

    log_section_header("End Test: Calculate Wallet Balance");
}

#[test]
fn test_empty_transaction_list() {
    // Test balance calculation with no transactions
    initialize("test_empty_transaction_list");
    log_section_header("Start Test: Empty Transaction List");

    let transactions = vec![];
    let balance = calculate_wallet_balance(&transactions, "wallet_3").unwrap();
    assert_eq!(balance, 0);

    log_section_header("End Test: Empty Transaction List");
}

#[test]
fn test_wallet_not_in_transactions() {
    // Test balance for wallet with no associated transactions
    initialize("test_wallet_not_in_transactions");
    log_section_header("Start Test: Wallet Not in Transactions");

    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: String::from("wallet_2"),
            amount: 100,
        },
    ];
    let balance = calculate_wallet_balance(&transactions, "wallet_3").unwrap();
    assert_eq!(balance, 0);

    log_section_header("End Test: Wallet Not in Transactions");
}

#[test]
fn test_multiple_deposits() {
    // Test balance calculation with multiple deposit transactions
    initialize("test_multiple_deposits");
    log_section_header("Start Test: Multiple Deposits");

    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: String::from("wallet_4"),
            amount: 100,
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: String::from("wallet_4"),
            amount: 200,
        },
    ];
    let balance = calculate_wallet_balance(&transactions, "wallet_4").unwrap();
    assert_eq!(balance, 300);

    log_section_header("End Test: Multiple Deposits");
}

#[test]
fn test_multiple_withdrawals() {
    // Test error handling for insufficient funds scenario
    initialize("test_multiple_withdrawals");
    log_section_header("Start Test: Multiple Withdrawals");

    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: String::from("wallet_5"),
            amount: 50,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: String::from("wallet_5"),
            amount: 30,
        },
    ];
    let result = calculate_wallet_balance(&transactions, "wallet_5");
    assert!(matches!(
        result,
        Err(WalletError::InsufficientFunds { .. })
    ));

    log_section_header("End Test: Multiple Withdrawals");
}

#[test]
fn test_mixed_transactions_multiple_wallets() {
    // Test balance tracking across multiple wallets with mixed transactions
    initialize("test_mixed_transactions_multiple_wallets");
    log_section_header("Start Test: Mixed Transactions Multiple Wallets");

    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: String::from("wallet_6"),
            amount: 150,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: String::from("wallet_6"),
            amount: 50,
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: String::from("wallet_7"),
            amount: 200,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: String::from("wallet_7"),
            amount: 100,
        },
    ];
    let balance_wallet_1 = calculate_wallet_balance(&transactions, "wallet_6").unwrap();
    let balance_wallet_2 = calculate_wallet_balance(&transactions, "wallet_7").unwrap();

    assert_eq!(balance_wallet_1, 100);
    assert_eq!(balance_wallet_2, 100);

    log_section_header("End Test: Mixed Transactions Multiple Wallets");
}

#[test]
fn test_invalid_transaction_amount() {
    // Test system handling of invalid transaction amounts
    initialize("test_invalid_transaction_amount");
    log_section_header("Start Test: Invalid Transaction Amount");

    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: String::from("wallet_8"),
            amount: -100,
        },
    ];

    let result = calculate_wallet_balance(&transactions, "wallet_8");
    assert!(matches!(result, Err(WalletError::InvalidAmount(-100))));

    log_section_header("End Test: Invalid Transaction Amount");
}

#[test]
fn test_insufficient_funds() {
    // Test withdrawal exceeding available balance
    initialize("test_insufficient_funds");
    log_section_header("Start Test: Insufficient Funds");

    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: String::from("wallet_9"),
            amount: 50,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: String::from("wallet_9"),
            amount: 100,
        },
    ];

    let result = calculate_wallet_balance(&transactions, "wallet_9");
    assert!(matches!(
        result,
        Err(WalletError::InsufficientFunds {
            requested: 100,
            available: 50
        })
    ));

    log_section_header("End Test: Insufficient Funds");
}

#[test]
fn test_display_transaction() {
    // Test transaction display formatting
    let transaction = Transaction {
        transaction_type: TransactionType::Deposit,
        wallet_address: String::from("wallet_1"),
        amount: 100,
    };
    assert_eq!(format!("{}", transaction), "Deposit of 100 to wallet_1");
}

#[test]
fn test_print_transaction_history() {
    // Test transaction history display functionality
    initialize("test_print_transaction_history");
    log_section_header("Start Test: Print Transaction History");

    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: String::from("wallet_1"),
            amount: 100,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: String::from("wallet_1"),
            amount: 30,
        },
    ];

    // Verify transaction display format
    let transaction = &transactions[0];
    assert_eq!(
        format!("{}", transaction),
        "Deposit of 100 to wallet_1"
    );

    let transaction = &transactions[1];
    assert_eq!(
        format!("{}", transaction),
        "Withdrawal of 30 to wallet_1"
    );

    // Verify history printing functionality
    print_transaction_history(&transactions, "wallet_1");

    log_section_header("End Test: Print Transaction History");
}

// Terminal-specific test module
mod terminal_tests {
    use super::*;
    use ryz_labs::terminal::WalletTerminal;

    // Initialize terminal instance for testing
    fn setup_terminal() -> WalletTerminal {
        initialize("test_terminal");
        WalletTerminal::new()
    }

    #[test]
    fn test_terminal_creation() {
        // Test terminal initialization
        let terminal = setup_terminal();
        let result = calculate_wallet_balance(&[], "test_wallet").unwrap();
        assert_eq!(result, 0);
    }

    // Helper function to process test transactions
    fn execute_transactions(transactions: Vec<Transaction>) -> Result<i64, WalletError> {
        let wallet_address = &transactions[0].wallet_address;
        calculate_wallet_balance(&transactions, wallet_address)
    }

    #[test]
    fn test_terminal_operations() {
        // Test basic terminal transaction operations
        let wallet = "test_wallet";
        let deposit_amount = 100;
        let withdrawal_amount = 30;

        let transactions = vec![
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: wallet.to_string(),
                amount: deposit_amount,
            },
            Transaction {
                transaction_type: TransactionType::Withdrawal,
                wallet_address: wallet.to_string(),
                amount: withdrawal_amount,
            },
        ];

        let final_balance = execute_transactions(transactions).unwrap();
        assert_eq!(final_balance, deposit_amount - withdrawal_amount);
    }

    #[test]
    fn test_terminal_insufficient_funds() {
        // Test terminal handling of insufficient funds
        let wallet = "test_wallet";
        
        let transactions = vec![
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: wallet.to_string(),
                amount: 50,
            },
            Transaction {
                transaction_type: TransactionType::Withdrawal,
                wallet_address: wallet.to_string(),
                amount: 100,
            },
        ];

        let result = execute_transactions(transactions);
        assert!(matches!(
            result,
            Err(WalletError::InsufficientFunds {
                requested: 100,
                available: 50
            })
        ));
    }

    #[test]
    fn test_terminal_multiple_wallets() {
        // Test terminal handling of multiple wallet operations
        let transactions = vec![
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: "wallet1".to_string(),
                amount: 100,
            },
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: "wallet2".to_string(),
                amount: 200,
            },
        ];

        let balance1 = calculate_wallet_balance(&transactions, "wallet1").unwrap();
        let balance2 = calculate_wallet_balance(&transactions, "wallet2").unwrap();

        assert_eq!(balance1, 100);
        assert_eq!(balance2, 200);
    }

    #[test]
    fn test_terminal_transaction_history() {
        // Test terminal transaction history functionality
        let wallet = "history_wallet";
        
        let transactions = vec![
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: wallet.to_string(),
                amount: 100,
            },
            Transaction {
                transaction_type: TransactionType::Withdrawal,
                wallet_address: wallet.to_string(),
                amount: 30,
            },
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: wallet.to_string(),
                amount: 50,
            },
        ];

        let balance = calculate_wallet_balance(&transactions, wallet).unwrap();
        assert_eq!(balance, 120);

        // Verify transaction display format
        let tx = &transactions[0];
        assert_eq!(
            format!("{}", tx),
            "Deposit of 100 to history_wallet"
        );
    }
} 
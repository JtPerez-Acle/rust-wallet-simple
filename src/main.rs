//! Entry point for the Ryz Labs Wallet Balance Tracker
//! Initializes and runs the interactive terminal interface

use ryz_labs::terminal::WalletTerminal;

/// Main function - initializes and runs the wallet terminal
fn main() {
    // Create new terminal instance
    let mut terminal = WalletTerminal::new();
    
    // Start the interactive terminal session
    terminal.run();
}

[![Join our Telegram](https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white)](https://t.me/hidden_coding)
[![GitHub](https://img.shields.io/badge/GitHub-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/aero25x)
[![Twitter](https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=x&logoColor=white)](https://x.com/aero25x)
[![YouTube](https://img.shields.io/badge/YouTube-FF0000?style=for-the-badge&logo=youtube&logoColor=white)](https://www.youtube.com/@flaming_chameleon)
[![Reddit](https://img.shields.io/badge/Reddit-FF3A00?style=for-the-badge&logo=reddit&logoColor=white)](https://www.reddit.com/r/HiddenCode/)


# Solana Wallet Topping Script


This Rust script facilitates transferring SOL (Solana) from your wallet to a specified recipient address. It utilizes the Solana RPC client to interact with the Solana blockchain and execute a SOL transfer transaction.

## Features

- **Dynamic Keypair Loading**: Automatically locates and loads the keypair file.
- **Balance Check**: Displays the sender's balance before proceeding with the transaction.
- **Configurable Transfer Amount**: Easily adjust the amount to be transferred.
- **Error Handling**: Comprehensive error handling for robust operation.
- **Transaction Confirmation**: Verifies and prints the transaction signature upon success.

## Prerequisites

Ensure the following are installed on your system:

- Rust (with `cargo` for package management)
- Solana CLI tools
- A Solana wallet with sufficient funds

## Setup Instructions

### Environment Variables

Set the `SOLANA_KEYPAIR` environment variable to point to your Solana wallet's keypair file. If this variable is not set, the script defaults to looking for the keypair file at `~/.config/solana/id.json`.

#### Example

```sh
export SOLANA_KEYPAIR=path/to/your/id.json
```

### Configuring the Script

Modify the following variables within the script to suit your requirements:

- **Recipient Address**: The Solana address to which the SOL will be sent.
- **Transfer Amount**: The amount of SOL to transfer (specified in lamports).

### Script Code

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
    pubkey::Pubkey,
    message::Message,
};
use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use dirs::home_dir;

fn main() {
    // Define the recipient address
    let recipient_address = "9VsY3Q9cPPiFf9984XJFXG7UEYdAUWpD1wuycvowprDm";

    // Load the keypair
    let keypair_path = std::env::var("SOLANA_KEYPAIR").unwrap_or_else(|_| {
        let mut home_dir = home_dir().expect("Unable to find home directory");
        home_dir.push(".config/solana/id.json");
        home_dir.to_string_lossy().to_string()
    });

    let keypair_file = File::open(&keypair_path).expect("Unable to open keypair file");
    let keypair_data: Value = serde_json::from_reader(keypair_file).expect("Unable to parse keypair file");
    let keypair_bytes: Vec<u8> = keypair_data.as_array()
        .expect("Expected array")
        .iter()
        .map(|v| v.as_u64().expect("Expected integer") as u8)
        .collect();
    let sender_keypair = Keypair::from_bytes(&keypair_bytes).expect("Invalid keypair bytes");

    println!("Sender pubkey: {}", sender_keypair.pubkey());

    // Create an RPC client
    let rpc_client = RpcClient::new("http://127.0.0.1:8899");

    // Get and print the sender's balance
    let sender_balance = rpc_client.get_balance(&sender_keypair.pubkey())
        .expect("Failed to get sender balance");

    println!("Sender balance: {} lamports", sender_balance);

    // Define the amount to transfer
    let transfer_amount = 10_000_000_000; // 10 SOL = 10_000_000_000 lamports

    // Check if the sender has sufficient balance
    if sender_balance < transfer_amount {
        eprintln!("Insufficient balance to perform the transfer");
        return;
    }

    // Get the recipient public key
    let recipient_pubkey = Pubkey::from_str(recipient_address).expect("Invalid recipient address");

    // Get a new blockhash for the transfer transaction
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get latest blockhash");

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(
        &sender_keypair.pubkey(),
        &recipient_pubkey,
        transfer_amount,
    );

    // Create the transaction
    let message = Message::new(&[transfer_instruction], Some(&sender_keypair.pubkey()));
    let transaction = Transaction::new(&[&sender_keypair], message, recent_blockhash);

    // Send the transaction
    let signature = match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => sig,
        Err(err) => {
            eprintln!("Failed to send transaction: {:?}", err);
            return;
        }
    };

    // Print the transaction signature
    println!("Transaction sent with signature: {}", signature);
}
```

### Code Explanation

1. **Imports**: Imports essential modules for interacting with Solana blockchain.
2. **Define Recipient Address**: Specifies the Solana address to receive the SOL.
3. **Load Keypair**: Loads the sender's keypair from a specified file.
4. **RPC Client**: Establishes a connection to the local Solana blockchain.
5. **Balance Check**: Retrieves and prints the sender's balance. Ensures sufficient balance before proceeding.
6. **Transfer Amount**: Specifies the amount of SOL to transfer.
7. **Create and Send Transaction**: Constructs and sends the transaction to the blockchain.
8. **Transaction Confirmation**: Outputs the transaction signature for verification.

## Running the Script

To run the script, use the following command:

```sh
cargo run
```

Ensure your Solana RPC server is running locally, or adjust the RPC URL as necessary.

## Advanced Configuration

### Custom RPC URL

You can modify the RPC URL to connect to a different Solana cluster by changing the `RpcClient` initialization:

```rust
let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com");
```

### Dynamic Transfer Amount

To dynamically specify the transfer amount, you can read it from an environment variable or a configuration file. Here’s an example of reading from an environment variable:

```rust
let transfer_amount: u64 = std::env::var("TRANSFER_AMOUNT")
    .expect("TRANSFER_AMOUNT not set")
    .parse()
    .expect("TRANSFER_AMOUNT must be a number");
```

## Troubleshooting

- **Keypair File Not Found**: Ensure the keypair file path is correct and the file exists.
- **Insufficient Funds**: Verify the sender's balance to ensure it has enough SOL.
- **Invalid Recipient Address**: Confirm that the recipient address is accurate.

For more information on Solana development, refer to the [Solana Documentation](https://docs.solana.com/).

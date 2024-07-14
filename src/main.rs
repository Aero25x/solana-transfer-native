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
use serde_json::Value;
use dirs::home_dir;

fn main() {
    // Define the recipient address (the one you want to top up)
    let recipient_address = "9VsY3Q9cPPiFf9984XJFXG7UEYdAUWpD1wuycvowprDm";

    // Define the private key (as a hex string) and convert it to bytes
    // let privkey_hex = "7c49dbdf76b6400b2d9343288eebc8d6906f278b53d5d78a12470c139c24ef9f7600fe03d82d5c7733e31380529827fc13be7d6478676f44ca73ef172063a2b9";
   
    // Get the keypair path
    let keypair_path = std::env::var("SOLANA_KEYPAIR").unwrap_or_else(|_| {
        let mut home_dir = home_dir().expect("Unable to find home directory");
        home_dir.push(".config/solana/id.json");
        home_dir.to_string_lossy().to_string()
    });

    // Load the keypair from the file
    let keypair_file = File::open(&keypair_path).expect("Unable to open keypair file");
    let keypair_data: Value = serde_json::from_reader(keypair_file).expect("Unable to parse keypair file");
    let keypair_bytes: Vec<u8> = keypair_data.as_array()
        .expect("Expected array")
        .iter()
        .map(|v| v.as_u64().expect("Expected integer") as u8)
        .collect();
    let sender_keypair = Keypair::from_bytes(&keypair_bytes).expect("Invalid keypair bytes");

    println!("Sender pubkey: {}", sender_keypair.pubkey());

    // Create an RPC client to interact with the Solana blockchain
    let rpc_client = RpcClient::new("http://127.0.0.1:8899");

    // Get and print the sender's balance
    let sender_balance = rpc_client
        .get_balance(&sender_keypair.pubkey())
        .expect("Failed to get sender balance");

    println!("Sender balance: {} lamports", sender_balance);

    // Define the amount to transfer (in lamports)
    let transfer_amount = 10_000_000_000; // 10 SOL = 10_000_000_000 lamports

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

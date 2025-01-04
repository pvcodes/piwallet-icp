use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

// Define a struct to represent a token wallet
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct TokenWallet {
    balances: HashMap<Principal, u64>, // Maps Principals (users) to their token balances
    owner: Principal,                  // The owner of the wallet, who has special privileges
}

// Implement a default constructor for TokenWallet
impl Default for TokenWallet {
    fn default() -> Self {
        TokenWallet {
            balances: HashMap::new(),      // Start with an empty balance map
            owner: Principal::anonymous(), // Default owner is anonymous, to be set later
        }
    }
}

// Define a struct for stable storage to persist wallet data across upgrades
#[derive(CandidType, Deserialize, Serialize)]
struct StableStorage {
    wallet: TokenWallet, // Contains the wallet data to be stored
}

// Thread-local storage for the wallet, ensuring each canister instance has its own wallet
thread_local! {
    static WALLET: RefCell<TokenWallet> = RefCell::new(TokenWallet::default());
}

// Define a result type for wallet operations, indicating success or failure
#[derive(CandidType, Deserialize)]
pub enum WalletResult {
    #[serde(rename = "ok")]
    Ok(String), // Success case with a descriptive message
    #[serde(rename = "err")]
    Err(String), // Error case with a descriptive message
}

// Initialization function for the canister, setting up the initial state
#[init]
fn init() {
    let owner = ic_cdk::api::caller(); // Get the caller's Principal to set as the owner
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        w.owner = owner; // Set the owner of the wallet to the caller
        w.balances.insert(owner, 1000000); // Initialize the owner's balance with a default amount
    });
}

// Pre-upgrade hook to save the current state before an upgrade
#[pre_upgrade]
fn pre_upgrade() {
    WALLET.with(|wallet| {
        let wallet_data = StableStorage {
            wallet: (*wallet.borrow()).clone(), // Clone the current wallet state
        };
        let bytes = Encode!(&wallet_data).expect("Failed to encode state"); // Encode the state for storage
        ic_cdk::storage::stable_save((bytes,)).expect("Failed to save state"); // Save the encoded state
    });
}

// Post-upgrade hook to restore the state after an upgrade
#[post_upgrade]
fn post_upgrade() {
    let (bytes,): (Vec<u8>,) = ic_cdk::storage::stable_restore().expect("Failed to restore bytes"); // Restore the saved state
    let StableStorage {
        wallet: wallet_data,
    } = Decode!(&bytes, StableStorage).expect("Failed to decode state"); // Decode the state

    WALLET.with(|wallet| {
        *wallet.borrow_mut() = wallet_data; // Restore the wallet data
    });
}

// Update method to send tokens from the caller to another Principal
#[update]
fn send_tokens(to: Principal, amount: u64) -> WalletResult {
    let caller = ic_cdk::api::caller(); // Get the caller's Principal

    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();

        let sender_balance = w.balances.get(&caller).copied().unwrap_or(0); // Get the sender's balance
        if sender_balance < amount {
            return WalletResult::Err("Insufficient balance".to_string()); // Check for sufficient balance
        }

        // Deduct the amount from the sender and add it to the recipient
        w.balances.insert(caller, sender_balance - amount);
        let recipient_balance = w.balances.get(&to).copied().unwrap_or(0);
        w.balances.insert(to, recipient_balance + amount);

        WalletResult::Ok(format!("Successfully sent {} tokens to {}", amount, to))
        // Return success message
    })
}

// Query method to get the balance of the caller
#[query]
fn get_balance() -> u64 {
    let caller = ic_cdk::api::caller(); // Get the caller's Principal
    WALLET.with(|wallet| wallet.borrow().balances.get(&caller).copied().unwrap_or(0))
    // Return the caller's balance
}

// Query method to check if an address exists in the wallet
#[query]
fn address_exists(address: Principal) -> bool {
    WALLET.with(|wallet| wallet.borrow().balances.contains_key(&address)) // Check for the existence of the address
}

// Query method to get the total supply of tokens in the wallet
#[query]
fn get_total_supply() -> u64 {
    WALLET.with(|wallet| wallet.borrow().balances.values().sum()) // Sum all balances to get the total supply
}

// Update method to mint new tokens, only callable by the owner
#[update]
fn mint(to: Principal, amount: u64) -> WalletResult {
    let caller = ic_cdk::api::caller(); // Get the caller's Principal

    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();

        if caller != w.owner {
            return WalletResult::Err("Only owner can mint tokens".to_string()); // Ensure only the owner can mint
        }

        let current_balance = w.balances.get(&to).copied().unwrap_or(0); // Get the current balance of the recipient
        w.balances.insert(to, current_balance + amount); // Add the minted amount to the recipient's balance

        WalletResult::Ok(format!("Successfully minted {} tokens to {}", amount, to))
        // Return success message
    })
}

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct TokenWallet {
    balances: HashMap<Principal, u64>,
    owner: Principal,
}

impl Default for TokenWallet {
    fn default() -> Self {
        TokenWallet {
            balances: HashMap::new(),
            owner: Principal::anonymous(),
        }
    }
}

#[derive(CandidType, Deserialize, Serialize)]
struct StableStorage {
    wallet: TokenWallet,
}

thread_local! {
    static WALLET: RefCell<TokenWallet> = RefCell::new(TokenWallet::default());
}

// Updated Result type to match Candid encoding
#[derive(CandidType, Deserialize)]
pub enum WalletResult {
    #[serde(rename = "ok")]
    Ok(String),
    #[serde(rename = "err")]
    Err(String),
}

#[init]
fn init() {
    let owner = ic_cdk::api::caller();
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        w.owner = owner;
        w.balances.insert(owner, 1000000);
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    WALLET.with(|wallet| {
        let wallet_data = StableStorage {
            wallet: (*wallet.borrow()).clone(),
        };
        let bytes = Encode!(&wallet_data).expect("Failed to encode state");
        ic_cdk::storage::stable_save((bytes,)).expect("Failed to save state");
    });
}

#[post_upgrade]
fn post_upgrade() {
    let (bytes,): (Vec<u8>,) = ic_cdk::storage::stable_restore().expect("Failed to restore bytes");
    let StableStorage { wallet: wallet_data } = Decode!(&bytes, StableStorage).expect("Failed to decode state");
    
    WALLET.with(|wallet| {
        *wallet.borrow_mut() = wallet_data;
    });
}

#[update]
fn send_tokens(to: Principal, amount: u64) -> WalletResult {
    let caller = ic_cdk::api::caller();
    
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        
        let sender_balance = w.balances.get(&caller).copied().unwrap_or(0);
        if sender_balance < amount {
            return WalletResult::Err("Insufficient balance".to_string());
        }

        // Deduct from sender and add to recipient
        w.balances.insert(caller, sender_balance - amount);
        let recipient_balance = w.balances.get(&to).copied().unwrap_or(0);
        w.balances.insert(to, recipient_balance + amount);
        
        WalletResult::Ok(format!("Successfully sent {} tokens to {}", amount, to))
    })
}

#[query]
fn get_balance() -> u64 {
    let caller = ic_cdk::api::caller();
    WALLET.with(|wallet| {
        wallet.borrow().balances.get(&caller).copied().unwrap_or(0)
    })
}

#[query]
fn address_exists(address: Principal) -> bool {
    WALLET.with(|wallet| {
        wallet.borrow().balances.contains_key(&address)
    })
}

#[query]
fn get_total_supply() -> u64 {
    WALLET.with(|wallet| {
        wallet.borrow().balances.values().sum()
    })
}

#[update]
fn mint(to: Principal, amount: u64) -> WalletResult {
    let caller = ic_cdk::api::caller();
    
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        
        if caller != w.owner {
            return WalletResult::Err("Only owner can mint tokens".to_string());
        }
        
        let current_balance = w.balances.get(&to).copied().unwrap_or(0);
        w.balances.insert(to, current_balance + amount);
        
        WalletResult::Ok(format!("Successfully minted {} tokens to {}", amount, to))
    })
}
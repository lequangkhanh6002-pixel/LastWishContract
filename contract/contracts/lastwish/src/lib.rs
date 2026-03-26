#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Address, Symbol, symbol_short
};

#[contracttype]
#[derive(Clone)]
pub struct Vault {
    pub owner: Address,
    pub beneficiary: Address,
    pub last_checkin: u64,
    pub timeout: u64,
    pub balance: i128,
}

#[contract]
pub struct LastWishContract;

const VAULT_KEY: Symbol = symbol_short!("VAULT");

#[contractimpl]
impl LastWishContract {

    // 🟢 Tạo vault
    pub fn init(
        env: Env,
        owner: Address,
        beneficiary: Address,
        timeout: u64,
    ) {
        owner.require_auth();

        let vault = Vault {
            owner: owner.clone(),
            beneficiary,
            last_checkin: env.ledger().timestamp(),
            timeout,
            balance: 0,
        };

        env.storage().instance().set(&VAULT_KEY, &vault);
    }

    // 💰 Nạp tiền (giả lập)
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let mut vault: Vault = env.storage().instance().get(&VAULT_KEY).unwrap();

        vault.balance += amount;

        env.storage().instance().set(&VAULT_KEY, &vault);
    }

    // 🔄 Check-in (gia hạn thời gian sống)
    pub fn checkin(env: Env, owner: Address) {
        owner.require_auth();

        let mut vault: Vault = env.storage().instance().get(&VAULT_KEY).unwrap();

        if owner != vault.owner {
            panic!("Not owner");
        }

        vault.last_checkin = env.ledger().timestamp();

        env.storage().instance().set(&VAULT_KEY, &vault);
    }

    // 🎯 Người thụ hưởng claim tiền
    pub fn claim(env: Env, beneficiary: Address) -> i128 {
        beneficiary.require_auth();

        let mut vault: Vault = env.storage().instance().get(&VAULT_KEY).unwrap();

        if beneficiary != vault.beneficiary {
            panic!("Not beneficiary");
        }

        let now = env.ledger().timestamp();

        if now < vault.last_checkin + vault.timeout {
            panic!("Too early to claim");
        }

        let amount = vault.balance;
        vault.balance = 0;

        env.storage().instance().set(&VAULT_KEY, &vault);

        amount
    }

    // 📊 Xem thông tin vault
    pub fn get_vault(env: Env) -> Vault {
        env.storage().instance().get(&VAULT_KEY).unwrap()
    }
}
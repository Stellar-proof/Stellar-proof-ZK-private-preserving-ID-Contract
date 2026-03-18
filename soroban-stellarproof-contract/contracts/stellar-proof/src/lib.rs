#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Bytes, BytesN, Env, Symbol
};
use soroban_sdk::xdr::ToXdr;
// ==============================
// STORAGE KEYS
// ==============================

#[contracttype]
pub enum DataKey {
    Identity(Address),       // user => identity_hash
    UsedNullifier(BytesN<32>), // nullifier => bool
    Admin,                   // contract admin
    Signer,                  // backend signer public key
}

// ==============================
//  CONTRACT
// ==============================

#[contract]
pub struct IdentityContract;

#[contractimpl]
impl IdentityContract {

    // ==============================
    // INIT
    // ==============================

    pub fn initialize(env: Env, admin: Address, signer: BytesN<32>) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Signer, &signer);
    }

    // ==============================
    // 🧍 REGISTER IDENTITY
    // ==============================

    pub fn register_identity(env: Env, user: Address, identity_hash: BytesN<32>) {
        user.require_auth();

        // store commitment
        env.storage().persistent().set(&DataKey::Identity(user.clone()), &identity_hash);
    }

    // ==============================
    // 🔍 VERIFY CLAIM (SIGNED RESULT)
    // ==============================

    pub fn verify_claim(
        env: Env,
        user: Address,
        identity_hash: BytesN<32>,
        nullifier: BytesN<32>,
        condition: Bytes,         // e.g. "OVER18"
        signature: BytesN<64>          // backend signature
    ) -> bool {

        // 1. Check identity exists
        let stored: Option<BytesN<32>> =
            env.storage().persistent().get(&DataKey::Identity(user.clone()));

        if stored.is_none() {
            panic!("identity not found");
        }

        if stored.unwrap() != identity_hash {
            panic!("invalid identity hash");
        }

        // 2. Prevent replay
        if env.storage().persistent().has(&DataKey::UsedNullifier(nullifier.clone())) {
            panic!("nullifier already used");
        }

        // 3. Reconstruct signed message
        let message = Self::build_message(&env, &user, &identity_hash, &nullifier, &condition);

        // 4. Verify signature
        let signer: BytesN<32> = env.storage().instance().get(&DataKey::Signer).unwrap();

         env.crypto().ed25519_verify(&signer, &message, &signature);

    
        // 5. Mark nullifier used
        env.storage().persistent().set(&DataKey::UsedNullifier(nullifier), &true);

        true
    }

    // ==============================
    // 🔧 INTERNAL: BUILD MESSAGE
    // ==============================

    fn build_message(
        env: &Env,
        user: &Address,
        identity_hash: &BytesN<32>,
        nullifier: &BytesN<32>,
        condition: &Bytes,
    ) -> Bytes {

        let mut msg = Bytes::new(env);

        msg.append(&user.to_xdr(env));
        msg.append(&identity_hash.clone().into());
        msg.append(&nullifier.clone().into());
        msg.append(condition);

        msg
    }

    // ==============================
    //  ADMIN: UPDATE SIGNER
    // ==============================

    pub fn update_signer(env: Env, new_signer: BytesN<32>) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        env.storage().instance().set(&DataKey::Signer, &new_signer);
    }

    // ==============================
    //  VIEW FUNCTIONS
    // ==============================

    pub fn get_identity(env: Env, user: Address) -> Option<BytesN<32>> {
        env.storage().persistent().get(&DataKey::Identity(user))
    }

    pub fn is_nullifier_used(env: Env, nullifier: BytesN<32>) -> bool {
        env.storage().persistent().has(&DataKey::UsedNullifier(nullifier))
    }
}
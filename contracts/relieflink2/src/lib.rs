#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address,
};

#[contracttype]
#[derive(Clone)]
pub struct ClaimRecord {
    pub resident: Address,
    pub amount: i128,
    pub claimed: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Claims(Address),
}

#[contract]
pub struct ReliefLinkContract;

#[contractimpl]
impl ReliefLinkContract {

    pub fn initialize_admin(env: Env, admin: Address) {
        admin.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
    }

    pub fn allocate_funds(
        env: Env,
        admin: Address,
        resident: Address,
        amount: i128,
    ) {

        admin.require_auth();

        let stored_admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != stored_admin {
            panic!("Unauthorized");
        }

        let claim = ClaimRecord {
            resident: resident.clone(),
            amount,
            claimed: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Claims(resident), &claim);
    }

    pub fn claim_relief(
        env: Env,
        resident: Address,
    ) {

        resident.require_auth();

        let mut claim: ClaimRecord = env.storage()
            .persistent()
            .get(&DataKey::Claims(resident.clone()))
            .unwrap();

        if claim.claimed {
            panic!("Already claimed");
        }

        claim.claimed = true;

        env.storage()
            .persistent()
            .set(&DataKey::Claims(resident), &claim);
    }

    pub fn check_claim_status(
        env: Env,
        resident: Address,
    ) -> bool {

        let claim: ClaimRecord = env.storage()
            .persistent()
            .get(&DataKey::Claims(resident))
            .unwrap();

        claim.claimed
    }

    pub fn get_balance(
        env: Env,
        resident: Address,
    ) -> i128 {

        let claim: ClaimRecord = env.storage()
            .persistent()
            .get(&DataKey::Claims(resident))
            .unwrap();

        claim.amount
    }
}

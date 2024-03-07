#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};
use crate::admin::{has_administrator, read_administrator, write_administrator};


#[contract]
pub struct AdminContract;

#[contractimpl]
impl AdminContract {
    pub fn initialize(e: Env, admin: Address) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
    }

    pub fn admin_sum(e: Env, a: i128, b: i128) {
        check_nonnegative_amount(amount);
        let admin = read_administrator(&e);
        admin.require_auth();

        a+b
    }
    
}

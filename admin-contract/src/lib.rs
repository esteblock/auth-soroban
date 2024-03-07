#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

mod admin;
mod storage_types;
mod test;

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

    pub fn admin_sum(e: Env, a: i128, b: i128) -> i128{
        let admin = read_administrator(&e);
        admin.require_auth();

        a+b
    }
    
}

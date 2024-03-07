#![cfg(test)]
extern crate std;

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal,
};

use crate::{AdminContract, AdminContractClient};

#[test]
#[should_panic(expected = "not yet initialized")]
fn admin_sum_not_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AdminContract);
    let client = AdminContractClient::new(&env, &contract_id);

    client.admin_sum(&1, &5);
}


#[test]
#[should_panic(expected = "already initialized")]
fn double_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AdminContract);
    let client = AdminContractClient::new(&env, &contract_id);
    let user_1 = Address::generate(&env);
    let user_2 = Address::generate(&env);

    client.initialize(&user_1);
    client.initialize(&user_2);
}


#[test]
fn admin_sum_mocked() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AdminContract);
    let client = AdminContractClient::new(&env, &contract_id);
    let user_2 = Address::generate(&env);

    client.initialize(&user_2);

    assert_eq!(client.admin_sum(&1, &5), 6);
}


#[test]
fn admin_sum() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AdminContract);
    let client = AdminContractClient::new(&env, &contract_id);
    let user_1 = Address::generate(&env);

    client.initialize(&user_1);

    client.admin_sum(&1, &5);
    // // Verify that the user indeed had to authorize a call of `increment` with
    // // the expected arguments:
    assert_eq!(
        env.auths(),
        std::vec![(
            // Address for which authorization check is performed
            user_1.clone(),
            // Invocation tree that needs to be authorized
            AuthorizedInvocation {
                // Function that is authorized. Can be a contract function or
                // a host function that requires authorization.
                function: AuthorizedFunction::Contract((
                    // Address of the called contract
                    contract_id.clone(),
                    // Name of the called function
                    symbol_short!("admin_sum"),
                    // Arguments used to call `increment` (converted to the env-managed vector via `into_val`)
                    (1_i128, 5_i128).into_val(&env),
                )),
                // The contract doesn't call any other contracts that require
                // authorization,
                sub_invocations: std::vec![]
            }
        )]
    );

}




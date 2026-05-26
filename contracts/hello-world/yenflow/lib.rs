#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Address,
    Env,
    Symbol,
};

#[contract]
pub struct YenFlowContract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Payment(u64),
}

#[contracttype]
#[derive(Clone)]
pub struct Payment {
    pub customer: Address,
    pub artist: Address,
    pub amount: i128,
    pub delivered: bool,
    pub released: bool,
}

#[contractimpl]
impl YenFlowContract {

    // Create commission escrow
    pub fn create_payment(
        env: Env,
        id: u64,
        customer: Address,
        artist: Address,
        amount: i128,
    ) {
        customer.require_auth();

        let payment = Payment {
            customer,
            artist,
            amount,
            delivered:false,
            released:false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Payment(id), &payment);
    }

    // Artist marks work complete
    pub fn mark_delivered(
        env: Env,
        id:u64,
        artist:Address
    ) {

        artist.require_auth();

        let mut payment:Payment = env
            .storage()
            .persistent()
            .get(&DataKey::Payment(id))
            .unwrap();

        if payment.artist != artist {
            panic!("Not artist");
        }

        payment.delivered=true;

        env.storage()
            .persistent()
            .set(
                &DataKey::Payment(id),
                &payment
            );
    }

    // Buyer confirms delivery
    pub fn release(
        env:Env,
        id:u64,
        customer:Address
    ){

        customer.require_auth();

        let mut payment:Payment=env
            .storage()
            .persistent()
            .get(&DataKey::Payment(id))
            .unwrap();

        if payment.customer!=customer{
            panic!("Unauthorized");
        }

        if !payment.delivered{
            panic!("Not delivered");
        }

        payment.released=true;

        env.storage()
            .persistent()
            .set(
                &DataKey::Payment(id),
                &payment
            );

    }

    pub fn get(
        env:Env,
        id:u64
    )->Payment{

        env.storage()
        .persistent()
        .get(&DataKey::Payment(id))
        .unwrap()
    }
}
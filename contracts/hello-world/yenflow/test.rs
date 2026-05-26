#[cfg(test)]

mod tests{

use super::*;
use soroban_sdk::{
testutils::{Address as _},
Address,
Env,
};

#[test]
fn happy_path(){

let env=Env::default();

let customer=Address::generate(&env);
let artist=Address::generate(&env);

YenFlowContract::create_payment(
env.clone(),
1,
customer.clone(),
artist.clone(),
25
);

YenFlowContract::mark_delivered(
env.clone(),
1,
artist.clone()
);

YenFlowContract::release(
env.clone(),
1,
customer.clone()
);

let payment=
YenFlowContract::get(env,1);

assert!(payment.released);

}

#[test]
#[should_panic]

fn unauthorized_release(){

let env=Env::default();

let customer=Address::generate(&env);

let artist=Address::generate(&env);

let stranger=Address::generate(&env);

YenFlowContract::create_payment(
env.clone(),
1,
customer,
artist.clone(),
25
);

YenFlowContract::mark_delivered(
env.clone(),
1,
artist
);

YenFlowContract::release(
env,
1,
stranger
);

}

#[test]

fn verify_storage(){

let env=Env::default();

let customer=Address::generate(&env);

let artist=Address::generate(&env);

YenFlowContract::create_payment(
env.clone(),
1,
customer.clone(),
artist.clone(),
50
);

let payment=
YenFlowContract::get(
env,
1
);

assert_eq!(
payment.amount,
50
);

}

#[test]

fn delivered_state(){

let env=Env::default();

let customer=Address::generate(&env);

let artist=Address::generate(&env);

YenFlowContract::create_payment(
env.clone(),
2,
customer,
artist.clone(),
25
);

YenFlowContract::mark_delivered(
env.clone(),
2,
artist
);

let p=
YenFlowContract::get(
env,
2
);

assert!(p.delivered);

}

#[test]

fn not_released_initially(){

let env=Env::default();

let customer=Address::generate(&env);

let artist=Address::generate(&env);

YenFlowContract::create_payment(
env.clone(),
3,
customer,
artist,
100
);

let p=
YenFlowContract::get(
env,
3
);

assert!(!p.released);

}

}
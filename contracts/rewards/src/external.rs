use near_sdk::ext_contract;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

// Interface of this contract, for callbacks
#[ext_contract(this_contract)]
trait Callbacks {
    fn query_greeting_callback(&self) -> String;
    fn change_greeting_callback(&self) -> bool;
}

// Validator interface, for cross-contract calls
#[ext_contract(hello_near)]
trait HelloNear {
    fn get_greeting(&self) -> String;
    fn set_greeting(&self, message: String);
}

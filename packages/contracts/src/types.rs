use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Neighborhood {
    pub name: String,
    pub verified: bool,
    pub admin: Address,
    pub member_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Post {
    pub author: Address,
    pub content: String,
    pub timestamp: u64,
}

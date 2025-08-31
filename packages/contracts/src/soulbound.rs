use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct SoulboundBadge;

#[contractimpl]
impl SoulboundBadge {
    // Issue a badge to a user. Only admin can call this.
    pub fn issue_badge(env: Env, to: Address) {
        to.require_auth();
        // Storage logic for badge balance would go here
        let key = Symbol::new(&env, "badge");
        env.storage().persistent().set(&key, &to);
    }

    // Check if a user has a badge
    pub fn has_badge(env: Env, user: Address) -> bool {
        // Logic to check verification
        true 
    }
    
    // Attempt to transfer (Always fails because it is Soulbound)
    pub fn transfer(_env: Env, _from: Address, _to: Address) {
        panic!("Soulbound: Transfer not allowed");
    }
}

#![no_std]
use soroban_sdk::{contract, contractimpl, log, token, Address, Env};

#[contract]
pub struct PaymentRouter;

#[contractimpl]
impl PaymentRouter {
    
    pub fn route_payment(
        env: Env,
        sender: Address,
        recipient: Address,         // For fiat withdrawals, this is the Anchor's wallet
        platform_treasury: Address,
        token_address: Address,     // The ID of the asset being sent (e.g., NGNC or USDC)
        amount: i128,          
        fee_percent: i128,     
    ) {
        // 1. Verify the sender authorized this transaction
        sender.require_auth();

        // 2. Calculate the split
        let fee_amount = (amount * fee_percent) / 100;
        let recipient_amount = amount - fee_amount;

        // 3. Initialize the token client for the specific currency
        let token_client = token::Client::new(&env, &token_address);

        // 4. Transfer the platform fee to your treasury
        // The client moves funds directly from the sender to the treasury
        token_client.transfer(&sender, &platform_treasury, &fee_amount);

        // 5. Transfer the remaining balance to the recipient (the Anchor)
        token_client.transfer(&sender, &recipient, &recipient_amount);

        // 6. Log success for testing
        log!(&env, "Platform fee routed to treasury");
        log!(&env, "Remaining balance routed to Anchor");
    }
}
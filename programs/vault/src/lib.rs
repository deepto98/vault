use anchor_lang::prelude::*;

declare_id!("8hX6Q2KWjpCNnvzih6mwkXAhVrRdQSsQH7eutbQ7j5Xo");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

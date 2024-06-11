use anchor_lang::prelude::*;

declare_id!("8hX6Q2KWjpCNnvzih6mwkXAhVrRdQSsQH7eutbQ7j5Xo");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer=user,
        seeds=[b"state",user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>, //vault state account to store bumps

    #[account(
         seeds=[b"vault",vault_state.key().as_ref()],
        bump,
     )]
    pub vault: SystemAccount<'info>, //actual vault account
    pub system_program: Program<'info, System>, // system program needed to initialize accounts
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        //InitializeBumps stores bumps for every acc in context - vault and vault_state
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}

// PDA Accounts
// Store bumps
#[account]
pub struct VaultState {
    pub vault_bump: u8, //save vault account pda's bump (referenced in Initialize)
    pub state_bump: u8, //save vault state pda's bump (this very struct)
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

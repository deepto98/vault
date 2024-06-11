use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };

declare_id!("8hX6Q2KWjpCNnvzih6mwkXAhVrRdQSsQH7eutbQ7j5Xo");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}

// 1. Initialize Context
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>, //vault state account to store bumps

    #[account(seeds = [b"vault", vault_state.key().as_ref()], bump)]
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
// 2. Deposit Context
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>, //user is needed to derive PDAs
    #[account(
        mut, // mutable because need to change the amount of lamports
        seeds = [b"state",vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>, //actual vault account

    #[account(seeds = [b"state", user.key().as_ref()], bump = vault_state.state_bump)]
    pub vault_state: Account<'info, VaultState>, //vault state account to store bumps

    pub system_program: Program<'info, System>, // system program needed send sol
}

impl<'info> Deposit<'info> {
    // Function to deposit funds to the vault using a CPI. System program transfers to vault
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        //amount will be passed from client

        // 1.Define CPI program - system program
        let cpi_program = self.system_program.to_account_info(); //get system program
        // 2.Create CPI accounts to and from
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };
        // 3. Create CPI Context (similar to usual context, but for system program)
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // 4.Transfer
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

// 3. Withdraw Context
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>, //user is needed to derive PDAs
    #[account(
        mut, // mutable because need to change the amount of lamports
        seeds = [b"state",vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>, //actual vault account

    #[account(seeds = [b"state", user.key().as_ref()], bump = vault_state.state_bump)]
    pub vault_state: Account<'info, VaultState>, //vault state account to store bumps

    pub system_program: Program<'info, System>, // system program needed send sol
}

// For user to vault - Deposit, the user signs the txn, so it is the signer
// For vault to user - Withdraw, the vault is a PDA, so the program has to sign on behalf of the PDA
// FOr that we need seeds
impl<'info> Withdraw<'info> {
    // Function to withdraw funds from the vault using a CPI. System program transfers from vault to user
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        //amount will be passed from client

        // 1.Define CPI program - system program
        let cpi_program = self.system_program.to_account_info(); //get system program
        // 2.Create CPI accounts to and from
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        // Seeds used to derive Vault PDA. The seeds sign on behalf of the vault PDA
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key().as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // 3. Create CPI Context (similar to usual context, but for system program)
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        // 4.Transfer
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

// 3. Close Context - Empty the Vault to the user and close state account(because no more lamports left )
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>, //user is needed to derive PDAs
    #[account(
        mut, // mutable because need to change the amount of lamports
        seeds = [b"state",vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>, //actual vault account

    #[account(
        mut, // mut because we'll close the vault_state PDA  
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user //while closing, rent is transferred from vault_state to user
        )]
    pub vault_state: Account<'info, VaultState>, //vault state account to store bumps

    pub system_program: Program<'info, System>, // system program needed send sol
}

impl<'info> Close<'info> {
    // Function to close
    pub fn close(&mut self) -> Result<()> {
        // 1.Define CPI program - system program
        let cpi_program = self.system_program.to_account_info(); //get system program
        // 2.Create CPI accounts to and from
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        // Seeds used to derive Vault PDA. The seeds sign on behalf of the vault PDA
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key().as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // 3. Create CPI Context (similar to usual context, but for system program)
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        // 4.Transfer
        transfer(cpi_ctx, self.vault.lamports())?;
        Ok(())
    }
}

// PDA Accounts
// Store bumps
#[account]
pub struct VaultState {
    pub vault_bump: u8, //save Vault   pda's bump (referenced in Initialize)
    pub state_bump: u8, //save VaultState pda's bump (this very struct)
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

use anchor_lang::prelude::*;

declare_id!("HAnmxVnFWCuQJ5jfYTmKSeiz1hwj25R2cDWps8rs6sGp");

#[program]
pub mod greet_svm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, greet:String) -> Result<()> {
        let account_data = &mut ctx.accounts.greet_account;
        // store the address of the `user`
        account_data.user = *ctx.accounts.user.key;
        // store the canonical bump
        account_data.bump = ctx.bumps.greet_account;
        // store the greet message
        account_data.greet = greet;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        // set the seeds to derive the PDA
        seeds = [b"data", user.key().as_ref()],
        // use the canonical bump
        bump,
        payer = user,
        space = 8 + GreetAccount::INIT_SPACE
    )]
    pub greet_account: Account<'info, GreetAccount>,
    pub system_program: Program<'info, System>,
}

#[account]

#[derive(InitSpace)]
pub struct GreetAccount {
    pub user: Pubkey,
    #[max_len(100)]
    pub greet: String,
    pub bump: u8,
}
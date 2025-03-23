use anchor_lang::prelude::*;

// !!! Replace with your program ID after running `anchor build` !!!
declare_id!("4qR1k6cpfrrpyj4tCHE9akCBfZHcNo1faFsQfuQbKsuJ");

#[program]
pub mod hello_sonic_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        greeting_account.counter = 0;
        greeting_account.authority = authority;
        Ok(())
    }

    pub fn increment_greeting(ctx: Context<IncrementGreeting>) -> Result<()> {
        msg!("Hello, Sonic World!");

        let greeting_account = &mut ctx.accounts.greeting_account;
        greeting_account.counter += 1;

        msg!("Greeted {} time(s)!", greeting_account.counter);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 4 + 32)]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncrementGreeting<'info> {
    #[account(mut, has_one = authority)]
    pub greeting_account: Account<'info, GreetingAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct GreetingAccount {
    pub counter: u32,
    pub authority: Pubkey,
}

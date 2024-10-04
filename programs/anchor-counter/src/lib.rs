use anchor_lang::prelude::*;

declare_id!("G2qX7dapyuzD6aSpLpRB2ahepxS819kktUSuMEZJv4Kg");

#[program]
pub mod anchor_counter {
    use super::*;
    // write our initialize instruction
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // initialize the counter account
        let counter = &mut ctx.accounts.counter;
        // set the variant field of the counter account to 0
        counter.count = 0;
        // log the value
        msg!("Counter Account Created. value is: {}", counter.count);
        // return the Ok variant of the result enum
        Ok(())
    }

    // implement the increment function here
    pub fn increment(ctx: Context<Update>) -> Result<()> {
        // initialize the counter account
        let counter = &mut ctx.accounts.counter;
        // setting the initial value of the count variant to 0
        counter.count = 0;
        // log the value
        msg!("Previous Counter value is: {}", counter.count);
        // increment by 3 using checked_add method
        counter.count = counter.count.checked_add(3).unwrap();
        msg!("Current counter value is : {}", counter.count);
        // return the Ok variant
        Ok(())
    }

    // implement the decrement function instruction here
    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        // initialize the counter count
        let counter = &mut ctx.accounts.counter;
        // setting the value of count to 3
        counter.count = 3;
        // log the message
        msg!("Counter value before decrement is: {}", counter.count);
        // decrement by 1 using checked_sub
        counter.count = counter.count.checked_sub(1).unwrap();
        // log the result after the decrement
        msg!("Counter value after decrement is: {}", counter.count);
        Ok(())

    }
}



#[account]
pub struct Counter {
    pub count: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

/* 
#[derive(Accounts)]
#[instruction(instruction_data: String)]
pub struct Example<'info> {
    #[account(
        seeds = [
            b"example_seed",
            user.key().as_ref(),
            instruction_data.as_ref()
        ],
        bump
    )]
    pub pda_account: Account<'info, AccountType>,
    #[account(mut)]
    pub user: Signer<'info>
}*/

// #NOTE the seeds and bump constraints can be combined with the init constraint to initialize an account using a PDA
// however, the init also must be used with the payer and space constraints.. the system_program must also be included to handle creation of new account
#[derive(Accounts)]
pub struct InitializePda<'info> {
    #[account(
        init,
        seeds = [b"example_seed", user.key().as_ref()],
        bump,
        payer = user,
        space = DISCRIMINATOR + AccountType::INIT_SPACE
    )]
    pub pda_account: Account<'info, AccountType>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct AccountType {
    pub data: u64,
}

const DISCRIMINATOR: usize = 8;


// USING THE init_if_needed constraint to initialize a new associated token account
#[derive(Accounts)]
pub struct Initialize2<'info > {
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}


// USING THE realloc constraint.... 
// The realloc constraint must be combined with mut, realloc::payer (account to subtract or add lamports to depending on whether reallocation is decreasing or increasing) 
//and realloc::zero (boolean to specify if new memory should be 0 initialized) constraints....

#[derive(Accounts)]
#[instruction(instruction_data: String)]
pub struct ReallocExample<'info> {
    #[account(
        mut,
        seeds = [b"example_seed"],
        bump,
        realloc = DISCRIMINATOR + STRING_SIZE_SPACE + instruction_data.len(),
        realloc::payer = user,
        realloc::zero = false,
    )]
    pub pda_account: Account<'info, AccountType>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct AccountType {
    pub data: String,
}

const DISCRIMINATOR: usize = 8;
const STRING_SIZE_SPACE: usize = 4;


// USING THE CLOSE constraint
// This works by setting the discriminator of the closed account to a special value called CLOSED_ACCOUNT_DISCRIMINATOR, and
// sends its lamports to a specified receiver account.. This special value prevents the account from being reopened cox
// any attempt to reinitialize the account will fail the discriminator check.. #see example below:
pub fn close(ctx: Context<Close>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = receiver)]
    pub data_account: Account<'info, AccountType>,
    #[account(mut)]
    pub receiver: Signer<'info>
}
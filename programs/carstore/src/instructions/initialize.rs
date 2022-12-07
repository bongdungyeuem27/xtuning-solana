use crate::schema::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = CarTypeAccount::SIZE,
        seeds = [b"car_type_account"],
        bump
    )]
    pub car_type_account: Account<'info, CarTypeAccount>,

    #[account(init, payer = user, space = CarAccount::SIZE, seeds = [b"car_account"], bump)]
    pub car_account: Account<'info, CarAccount>,

    #[account(init, payer = user, space = UserAccount::SIZE, seeds = [b"user_account"], bump)]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init,
        payer = user,
        space = CarStoreAccount::SIZE,
        seeds = [b"car_store_account"],
        bump
    )]
    pub car_store_account: Account<'info, CarStoreAccount>,

    // System Program Address
    pub system_program: Program<'info, System>,
    // pub token_program: Program<'info, token::Token>,
    // pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    // pub rent: Sysvar<'info, Rent>,
}

pub fn exec(
    ctx: Context<Initialize>,
    car_type_account_bump: u8,
    car_account_bump: u8,
    user_account_bump: u8,
    car_store_account_bump: u8
) -> Result<()> {
    let owner = ctx.accounts.user.key().clone();

    let car_type_account = &mut ctx.accounts.car_type_account;
    let car_account = &mut ctx.accounts.car_account;
    let user_account = &mut ctx.accounts.user_account;
    let car_store_account = &mut ctx.accounts.car_store_account;

    car_type_account.owner = owner.clone();
    car_type_account.bump = car_type_account_bump.clone();
    car_type_account.list = Vec::new();

    car_account.bump = car_account_bump.clone();
    car_account.list = Vec::new();
    car_account.owner = owner.clone();

    user_account.bump = user_account_bump.clone();
    user_account.list = Vec::new();
    user_account.owner = owner.clone();

    car_store_account.bump = car_store_account_bump.clone();
    car_store_account.owner = owner.clone();

    Ok(())
}
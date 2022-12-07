use crate::schema::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct AddCar<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
      mut,
      seeds = [b"car_account"],
      bump  
    )]
    pub car_account: Account<'info, CarAccount>,

    #[account(
        mut,
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

pub fn exec(ctx: Context<AddCar>, car_type_id: u64) -> Result<()> {
    let item = Car {
        car_id: ctx.accounts.car_account.list.len() as u64,
        car_type_id,
        car_available: true,
        car_owner: ctx.accounts.car_store_account.owner.clone(),
    };
    ctx.accounts.car_account.list.push(item);
    Ok(())
}
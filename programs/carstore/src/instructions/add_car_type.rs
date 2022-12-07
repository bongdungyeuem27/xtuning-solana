use crate::schema::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct AddCarType<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
      mut,
      seeds = [b"car_type_account"],
      bump  
    )]
    pub car_type_account: Account<'info, CarTypeAccount>,

    // System Program Address
    pub system_program: Program<'info, System>,
    // pub token_program: Program<'info, token::Token>,
}

pub fn exec(
    ctx: Context<AddCarType>,
    car_type_name: String,
    car_type_description: String,
    car_brand_name: String,
    car_type_price: u64,
) -> Result<()> {
    let item = CarType {
        car_type_id: ctx.accounts.car_type_account.list.len() as u64,
        car_type_name,
        car_type_description,
        car_brand_name,
        car_type_price,
    };
    ctx.accounts.car_type_account.list.push(item);
    Ok(())
}

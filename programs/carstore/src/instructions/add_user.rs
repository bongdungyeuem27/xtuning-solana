use crate::schema::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
      mut,
      seeds = [b"user_account"],
      bump
    )]
    pub user_account: Account<'info, UserAccount>,
    // System Program Address
    pub system_program: Program<'info, System>,
}

pub fn exec(ctx: Context<AddUser>, user_name: String, user_address: String) -> Result<()> {
    let item = User {
        user_id: ctx.accounts.user_account.list.len() as u64,
        user_name,
        user_address,
        user_owner: ctx.accounts.user.key(),
    };
    ctx.accounts.user_account.list.push(item);
    Ok(())
}
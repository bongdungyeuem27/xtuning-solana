use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct BuyCar<'info> {
    /// CHECK: check signer unsafe
    #[account(mut, signer @ ErrorCode::NotSigner)]
    pub payer_account: AccountInfo<'info>,
    /// CHECK: check signer unsafe
    #[account(mut)]
    pub reciver_account: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"user_account"],
        bump
      )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        seeds = [b"car_account"],
        bump
      )]
    pub car_account: Account<'info, CarAccount>,
    #[account(seeds = [b"car_type_account"], bump)]
    pub car_type_account: Account<'info, CarTypeAccount>,

    #[account(
        mut,
        seeds = [b"car_store_account"],
        bump
    )]
    pub car_store_account: Account<'info, CarStoreAccount>,
    // System Program Address
    pub system_program: Program<'info, System>,
}

pub fn exec(ctx: Context<BuyCar>, car_id: u64) -> Result<()> {
    emit!(MyEvent {
        data: 5,
        label: [1, 2, 3, 4, 5],
        owner: ctx.accounts.reciver_account.key(),
    });

    let owner = ctx.accounts.car_store_account.owner.clone();
    let payer_account = &mut ctx.accounts.payer_account;
    let reciver_account = &mut ctx.accounts.reciver_account;
    require!(owner == reciver_account.key(), ErrorCode::ReciverAccountNotowner);

    let list_user = &ctx.accounts.user_account.list;
    let has_user = list_user.iter().any(|i| i.user_owner == payer_account.key());
    require!(has_user, ErrorCode::RequireUser);

    let list_car = &mut ctx.accounts.car_account.list;
    let car_pos = list_car
        .iter()
        .position(|i| i.car_id == car_id)
        .unwrap();
    let car = &mut (*list_car)[car_pos];
    let car_type = ctx.accounts.car_type_account.list
        .iter()
        .find(|&i| i.car_type_id == car.car_type_id)
        .unwrap();

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: payer_account.clone(),
            to: reciver_account.clone(),
        }
    );
    system_program::transfer(cpi_context, car_type.car_type_price)?;
    car.car_available = false;
    car.car_owner = payer_account.key().clone();
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("buy_car: require user")]
    RequireUser,
    #[msg("buy_car: not signer")]
    NotSigner,
    #[msg("buy_car: rerciver account not owner")]
    ReciverAccountNotowner,
}

#[event]
pub struct MyEvent {
    pub data: u64,
    pub label: [u8; 5],
    pub owner: Pubkey,
}
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct User {
    pub user_id: u64,
    pub user_name: String,
    pub user_address: String,
    pub user_owner: Pubkey,
}

#[account]
pub struct UserAccount {
    pub list: Vec<User>,
    pub bump: u8,
    pub owner: Pubkey,
}

impl UserAccount {
    pub const SIZE: usize = 8 + 512;
}
use anchor_lang::prelude::*;

#[account]
pub struct CarStoreAccount {
    pub bump: u8,
    pub owner: Pubkey,
}

impl CarStoreAccount {
    pub const SIZE: usize = 8 + 64;
}
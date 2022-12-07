use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Car {
    pub car_id: u64,
    pub car_type_id: u64,
    pub car_available: bool,
    pub car_owner: Pubkey,
}

#[account]
pub struct CarAccount {
    pub list: Vec<Car>,
    pub bump: u8,
    pub owner: Pubkey,
}

impl CarAccount {
    pub const SIZE: usize = 8 + 512;
}
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CarType {
    pub car_type_id: u64,
    pub car_type_name: String,
    pub car_type_description: String,
    pub car_brand_name: String,
    pub car_type_price: u64,
}

#[account]
pub struct CarTypeAccount {
    pub list: Vec<CarType>,
    pub bump: u8,
    pub owner: Pubkey,
}

impl CarTypeAccount {
    pub const SIZE: usize = 8 + 512;
}
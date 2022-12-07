use anchor_lang::prelude::*;

declare_id!("G7XMcMqA1163mYCQdLwoTaBA9PYS1eZePViaRnWprc9f");
pub mod schema;
pub use schema::*;

pub mod instructions;
pub use instructions::*;

#[program]
pub mod carstore {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        car_type_account_bump: u8,
        car_account_bump: u8,
        user_account_bump: u8,
        car_store_info_bump: u8
    ) -> Result<()> {
        initialize::exec(
            ctx,
            car_type_account_bump,
            car_account_bump,
            user_account_bump,
            car_store_info_bump
        )
    }

    pub fn add_car_type(
        ctx: Context<AddCarType>,
        car_type_name: String,
        car_type_description: String,
        car_brand_name: String,
        car_type_price: u64
    ) -> Result<()> {
        add_car_type::exec(ctx, car_type_name, car_type_description, car_brand_name, car_type_price)
    }

    pub fn add_car(ctx: Context<AddCar>, car_type_id: u64) -> Result<()> {
        add_car::exec(ctx, car_type_id)
    }

    pub fn add_user(ctx: Context<AddUser>, user_name: String, user_address: String) -> Result<()> {
        add_user::exec(ctx, user_name, user_address)
    }

    pub fn buy_car(ctx: Context<BuyCar>, car_id: u64) -> Result<()> {
        buy_car::exec(ctx, car_id)
    }
}
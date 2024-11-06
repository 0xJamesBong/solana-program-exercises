#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("FGAFbELLrEzXa1JECooVR2UVMaK1bkgTsdZv7DBzvvBU");

#[program]
pub mod account_data {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // pub fn create_address_info(
    //     ctx: Context<CreateAddressInfo>,
    //     name: String,
    //     house_number: u8,
    //     street: String,
    //     city: String,
    // ) -> Result<()> {
    //     create::create_address_info(ctx, name, house_number, street, city)
    // }
}

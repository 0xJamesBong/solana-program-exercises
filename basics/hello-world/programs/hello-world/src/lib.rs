use anchor_lang::prelude::*;

declare_id!("Eub9MuruafRZbPzcmYzjNMGYtCnGm1GknoGJHfx2dfSy");

#[program]
pub mod hello_world {
    use super::*;

    pub fn hello_world(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world!");
        Ok(())
    }

    pub fn merry_christmas(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Merry Christmas you filthy animal!");
        Ok(())
    }
    pub fn happy_new_year(_ctx: Context<Initialize>) -> Result<()> {
        msg!("And a Happy New Year!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

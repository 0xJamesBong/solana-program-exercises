use anchor_lang::prelude::*;

declare_id!("BRdMHmUPn5QRA6iGyuEVLuBCs8Z9MxdtwcW25kwJBRKa");

#[program]
pub mod basic_template {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

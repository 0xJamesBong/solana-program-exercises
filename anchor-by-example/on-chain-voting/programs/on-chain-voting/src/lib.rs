use anchor_lang::prelude::*;

declare_id!("4c1xHGDjPz6DMzDRvTedYJzUc4yAdKbsYRSFNxbQrSSr");

#[program]
// pub mod on_chain_voting {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         msg!("Greetings from: {:?}", ctx.program_id);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

pub mod on_chain_voting {
    use super::*;
    pub fn init_vote_bank(ctx: Context<InitVote>) -> Result<()> {
        // Open vote bank for public to vote on our favourite "GM" or "GN"
        ctx.accounts.vote_account.is_open_to_vote = true;
        Ok(())
    }

    pub fn gib_vote(ctx: Context<GibVote>, vote_type: VoteType) -> Result<()> {
        // If vote_type is GM increment GM by 1 else increment GN by 1
        match vote_type {
            VoteType::Gm => {
                msg!("Voted for GM  🤝");
                ctx.accounts.vote_account.gm += 1;
            }
            VoteType::GN => {
                msg!("Voted for GN  🤞");
                ctx.accounts.vote_account.gn += 1;
            }
        };
        Ok(())
    }
}

#[derive(Accounts)]

pub struct InitVote<'info> {
    // Making a global account for storing votes
    #[account(init, payer = signer, space = 8+1+8+8,)]
    pub vote_account: Account<'info, VoteBank>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct VoteBank {
    pub is_open_to_vote: bool,
    pub gm: u64,
    pub gn: u64,
}

#[derive(Accounts)]
pub struct GibVote<'info> {
    // we are going to store users vote in this account. Hence marking it as mutable (mut)
    #[account(mut)]
    pub vote_account: Account<'info, VoteBank>,
    pub signer: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    Gm,
    GN,
}

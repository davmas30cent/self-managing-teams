use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("35FMbNaP6dW2sk8Qotgn2MrdyGZYK5hmSfz93ce9wdUg");

#[program]
pub mod self_managing_teams {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn add_myself(ctx: Context<AddMyself>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
              payer = signer,
              space = size_of::<TeamMembers>() + size_of::<TeamMember>()*5 + 8)]
    pub team_members: Account<'info, TeamMembers>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddMyself {

}

#[account]
pub struct TeamMembers {
    members: Vec<TeamMember>
}

#[account]
pub struct TeamMember {
    name: String,
    role: String,
    pubkey: String
}

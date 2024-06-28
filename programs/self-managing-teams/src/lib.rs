use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("35FMbNaP6dW2sk8Qotgn2MrdyGZYK5hmSfz93ce9wdUg");

#[program]
pub mod self_managing_teams {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_team_member_account(ctx: Context<CreateTeamMember>,
                                      name: String,
                                      role: String) -> Result<()> {
        ctx.accounts.team_member.name = name;
        ctx.accounts.team_member.role = role;
        ctx.accounts.team_member.pubkey = ctx.accounts.team_member.pubkey;
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
pub struct CreateTeamMember<'info> {
    #[account(init,
              payer = signer,
              space = size_of::<TeamMember>() + 8)]
    pub team_member: Account<'info, TeamMember>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct TeamMembers {
    pub members: Vec<TeamMember>
}

#[account]
pub struct TeamMember {
    pub name: String,
    pub role: String,
    pub pubkey: Pubkey
}

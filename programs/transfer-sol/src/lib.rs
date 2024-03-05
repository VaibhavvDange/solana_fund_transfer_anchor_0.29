use anchor_lang::prelude::*;

declare_id!("2QpQyZ2k4fZoNEGgi4suwbiEsQcBsGpnxWJgfxUxJ2z2");

#[program]
pub mod transfer_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct StoreFunds<'info,Counter> {
    #[account(mut)]
    pub user: Signer<'info>,
    // space: 8 discriminator + 2 level + 4 name length + 200 name + 1 bump
    #[account(
        init,
        payer = user,
        space = 8 + 2 + 4 + 200 + 1, seeds = [b"user-stats", user.key().as_ref()], bump
    )]
    pub balance: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit{
    pub user : Signer<User>
}


#[account]
pub struct User{

}
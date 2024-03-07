use anchor_lang::prelude::*;

declare_id!("2QpQyZ2k4fZoNEGgi4suwbiEsQcBsGpnxWJgfxUxJ2z2");

#[program]
pub mod transfer_sol {
    use anchor_lang::{accounts::signer, solana_program::{self, log, system_instruction, vote::program}, Bump};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("🚀 ~ initialize method invoked");
        // let treasury_account = &mut ctx.accounts.treasury;
        Ok(())
    }

    pub fn deposit_lamports(ctx: Context<DepositLamports>,amount: u64) -> Result<()>{
        msg!("🚀 ~ deposit_lamports method invoked");

        let from_account = &ctx.accounts.user;
        let to_account = &ctx.accounts.treasury;

        msg!("🚀 ~ pubfndeposit_lamports ~ from_account: {:?}", from_account.key);
        msg!("🚀 ~ pubfndeposit_lamports ~ to_account: {:?}", to_account.key());

        // Create the transfer instruction
        let transfer_instruction = system_instruction::transfer(from_account.key, &to_account.key(), amount);

        // Invoke the transfer instruction
        solana_program::program::invoke(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
        
    }

    pub fn withdraw_lamports(ctx: Context<WithdrawLamports>, amount: u64) -> Result<()> {
        msg!("Funds to be withdraw: {}", amount);
        msg!("🚀 ~ withdraw_lamports method invoked");

        let from_account = &ctx.accounts.treasury;
        let to_account = &ctx.accounts.user;

        msg!("Balance in treasury: {}", from_account.get_lamports());

        msg!("🚀 ~ pubfnwithdraw_lamports ~ from_account: {:?}", from_account.key());
        msg!("🚀 ~ pubfnwithdraw_lamports ~ to_account: {:?}", to_account.key);

        let treasury_seed = b"treasury"; 
        let program_id = ctx.program_id;

        let (pda_address, pda_bump) = Pubkey::find_program_address(&[treasury_seed], &program_id);
        msg!("🚀 ~ pubfnwithdraw_lamports ~ pda_address: {:?}", pda_address.key());
        msg!("pda_bump : {:?}", pda_bump);

        // let bump = *ctx.bumps.get("treasury").unwrap()
        let pda_account = ctx.accounts.treasury.to_account_info();
        let send_to_account = ctx.accounts.user.to_account_info();

        **pda_account.try_borrow_mut_lamports()? -= amount;
        **send_to_account.try_borrow_mut_lamports()? += amount;

        // // Create the transfer instruction
        // let transfer_instruction = system_instruction::transfer(&pda_address.key() , ctx.accounts.user.key, amount);

        // // Invoke the transfer instruction
        // anchor_lang::solana_program::program::invoke_signed(
        //     &transfer_instruction,
        //     &[
        //         from_account.to_account_info(),
        //         to_account.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        //     &[
        //         &[
        //             b"treasury".as_ref(), // since signing using PDA, therefore passing signers seeds
        //             &[pda_bump],
        //         ]
        //     ],
        //     )?;
            msg!("Executed!!");
        Ok(())
    }

    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 10000 ,seeds = [b"treasury"],bump)]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub user : Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositLamports<'info> {
    #[account(mut, 
     seeds = [b"treasury"], bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub user : Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawLamports<'info> {
    #[account(mut,
     seeds = [b"treasury"], bump
    )]
    pub treasury: Account<'info , Treasury>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub user: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Treasury {
    pub bump: u8,
}

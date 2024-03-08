use anchor_lang::prelude::*;

declare_id!("GkXjAM7x9QSkWaPZeNaVPp1csuneuLonAK1Kwxv8gN8i");

#[program]
pub mod transfer_sol {
    use anchor_lang::solana_program::{self, system_instruction};

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("🚀 ~ initialize method invoked");
        // let treasury_account = &mut ctx.accounts.treasury;
        Ok(())
    }

    pub fn initialize2(_ctx: Context<Initialize2>) -> Result<()> {
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

    pub fn deposit_lamports2(ctx: Context<DepositLamports2>,amount: u64) -> Result<()>{
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
        let depositor_length = to_account.depositors_length;
        msg!("🚀 ~ pubfndeposit_lamports2 ~ depositor_length: {:?}", depositor_length);
        for x in 0..10{
            if x == depositor_length {
                // let y : Bytes<> = x.into();
                ctx.accounts.treasury.depositors_pubkey[x as usize] = from_account.key();
                ctx.accounts.treasury.depositors_length += 1;
            }else if depositor_length > 9 {
                msg!("Array of depositor's list is Full !!");
                break;
            }
        }
        let depositors_list = ctx.accounts.treasury.depositors_pubkey;
        msg!("🚀 ~ pubfndeposit_lamports2 ~ depositors_list: {:?}", depositors_list);
        
        // ctx.accounts.treasury.depositors_pubkey[1] = from_account.key();
        // msg!("🚀 ~ pubfndeposit_lamports2 ~ ctx.accounts.treasury.depositors_pubkey[1] : {}", ctx.accounts.treasury.depositors_pubkey[1] );
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

    pub fn withdraw_lamports2(ctx: Context<WithdrawLamports2>, amount: u64) -> Result<()> {
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
        let withdrawer_length = from_account.withdrawer_length;
        msg!("🚀 ~ pubfnwithdraw_lamports2 ~ withdrawer_length: {:?}", withdrawer_length);
        for x in 0..10{
            if x == withdrawer_length {
                // let y : Bytes<> = x.into();
                ctx.accounts.treasury.withdrawer_pubkey[x as usize] = to_account.key();
                ctx.accounts.treasury.withdrawer_length += 1;
            }else if withdrawer_length > 9 {
                msg!("Array of withdrawer list is Full !! ");
                break;
            }
        }
        let withdrawer_list = ctx.accounts.treasury.withdrawer_pubkey;
        msg!("🚀 ~ pubfnwithdraw_lamports2 ~ withdrawer_list: {:?}", withdrawer_list);


            // ctx.accounts.treasury.withdrawer_pubkey.push(to_account.key());
        msg!("🚀 ~ pubfndeposit_lamports2 ~ ctx.accounts.treasury.withdrawer_pubkey[1] : {}", ctx.accounts.treasury.depositors_pubkey[1] );
          
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

#[account]
pub struct Treasury2 {
    pub depositors_pubkey : [Pubkey;10],
    pub depositors_length : u8,
    pub withdrawer_pubkey : [Pubkey;10],
    pub withdrawer_length : u8,

}

#[derive(Accounts)]
pub struct Initialize2<'info> {
    #[account(init, payer = user, space = 10000 ,seeds = [b"treasury"],bump)]
    pub treasury: Account<'info, Treasury2>,

    #[account(mut)]
    pub user : Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositLamports2<'info> {
    #[account(mut, 
        seeds = [b"treasury"], bump
    )]
    pub treasury: Account<'info, Treasury2>,

    #[account(mut)]
    pub user : Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawLamports2<'info> {
    #[account(mut,
     seeds = [b"treasury"], bump
    )]
    pub treasury: Account<'info , Treasury2>,

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub user: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}
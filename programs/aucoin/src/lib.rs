use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("re1SvbkNVZbbY2jJEL5ikASniGDB7s7y6E6mrzGFS38");

#[program]
pub mod aucoin {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String) -> ProgramResult{
        let collectable = &mut ctx.accounts.collectable;
        collectable.name = name;
        collectable.description = description;
        collectable.amount_d = 0;
        collectable.admin = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Whithdraw>, amount: u64) -> ProgramResult{
        let collectable = &mut ctx.accounts.collectable;
        let user = &mut ctx.accounts.user;

        if collectable.admin != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }
        
        let rent_balance = Rent::get()?.minimum_balance(collectable.to_account_info().data_len());
        
        if **collectable.to_account_info().lamports.borrow()- rent_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        **collectable.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    pub fn bind(ctx: Context<Bind>, amount: u64) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.collectable.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.collectable.to_account_info()
            ]
        );

        (&mut ctx.accounts.collectable).amount_d += amount;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=9000, seeds=[b"COLLECTABLE_DEMO".as_ref(), user.key().as_ref()],bump)]
    pub collectable: Account<'info, Collectable>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Whithdraw<'info>{
    #[account(mut)]
    pub collectable: Account<'info, Collectable>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct Bind<'info>{
    #[account(mut)]
    pub collectable: Account<'info, Collectable>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Collectable {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub amount_d:u64
}
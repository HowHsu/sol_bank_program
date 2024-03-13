use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use std::str::FromStr;

declare_id!("A2S93vbFoLvkeYmWKtpTWnbA9yKmtSAmBq7GFYQnRsav");

const DEPLOYER_PUBKEY_STR: &str = "8pWXLmFkVae27kPiympG2R6JZytmeK9xjQ8SndYKwEvD";

#[program]
pub mod bank_contract {
    use super::*;

    pub fn set_admin(ctx: Context<SetAdmin>, admin: Pubkey) -> Result<()> {
        let bank_owner = &mut ctx.accounts.bank_owner;
        bank_owner.admin = admin;
        msg!("set_admin suceed, admin: {}", admin);
        Ok(())
    }

    pub fn enable_token(ctx: Context<EnableToken>) -> Result<()> {
        msg!("enable token: {}", ctx.accounts.mint.key());
        Ok(())
    }

    pub fn disable_token(ctx: Context<DisableToken>) -> Result<()> {
        msg!("disable token: {}", ctx.accounts.mint.key());
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        msg!(
            "deposit: user: {}, token: {}, amount: {}",
            ctx.accounts.owner.key,
            ctx.accounts.from.mint,
            amount
        );
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // todo: should consider mint.decimal???
        require!(
            ctx.accounts.from.amount >= amount,
            BankError::WithdrawTooMuch
        );
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.from.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        msg!(
            "withdraw: user: {}, token: {}, to: {}, amount: {}",
            ctx.accounts.owner.key,
            ctx.accounts.from.mint,
            ctx.accounts.to.key(),
            amount
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetAdmin<'info> {
    #[account(
        init_if_needed,
        payer = deployer,
        seeds = [b"bank_owner"],
        bump,
        space = 8 + 32 + 32 + 1
    )]
    pub bank_owner: Account<'info, BankOwner>,
    #[account(mut, address = Pubkey::from_str(DEPLOYER_PUBKEY_STR).unwrap())]
    pub deployer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnableToken<'info> {
    #[account(mut, seeds = [b"bank_owner"], bump)]
    pub bank_owner: Account<'info, BankOwner>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [mint.key().as_ref()],
        bump,
        space = 8 + 1
    )]
    pub token_config: Account<'info, TokenConfig>,
    #[account(mut, address = bank_owner.admin)]
    pub admin: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DisableToken<'info> {
    #[account(mut, seeds = [b"bank_owner"], bump)]
    pub bank_owner: Account<'info, BankOwner>,
    #[account(
        mut,
        close = admin,
        seeds = [mint.key().as_ref()],
        bump,
    )]
    pub token_config: Account<'info, TokenConfig>,
    #[account(mut, address = bank_owner.admin)]
    pub admin: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    // should have checked token::mint and token::authority but didn't make it
    #[account(
        init_if_needed,
        seeds = [b"contract_token_account", owner.key.as_ref(), from.mint.as_ref()],
        bump,
        payer = owner,
        space = TokenAccount::LEN,
    )]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [from.mint.as_ref()], bump)]
    pub token_config: Account<'info, TokenConfig>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"contract_token_account", owner.key.as_ref(), to.mint.as_ref()],
        bump,
    )]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct BankOwner {
    pub admin: Pubkey,
}

#[account]
pub struct TokenConfig {}

#[error_code]
pub enum BankError {
    #[msg("withdraw amount larger than deposited")]
    WithdrawTooMuch,
}

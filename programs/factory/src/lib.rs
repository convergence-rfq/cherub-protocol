//! An example of an AMM factory program, inspired by Uniswap V1 seen here:
//! https://github.com/Uniswap/uniswap-v1/. This example has some
//! implementation changes to address the differences between the EVM and
//! Solana's BPF-modified LLVM, but more or less should be the same overall.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::TokenAccount;

use exchange::cpi::accounts::Create;
use exchange::program::Exchange;
use exchange::{self, ExchangeData};

declare_id!("FyuPaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Factory
#[program]
pub mod factory {
    use super::*;

    /// Initializes the factory account
    pub fn initialize(ctx: Context<Initialize>, template: Pubkey) -> ProgramResult {
        //let factory = &mut ctx.accounts.factory;
        //factory.exchange_template = template;
        //factory.token_count = 0;
        Ok(())
    }

    /// Creates an exchange for a new token pair
    pub fn create_exchange(
        ctx: Context<CreateExchange>,
        token_a: Pubkey,
        token_b: Pubkey,
        token_c: Pubkey,
        fee: u64,
    ) -> ProgramResult {
        let factory = &mut ctx.accounts.factory.clone();
        factory.token_count = factory.token_count + 1;
        //let seeds = &[factory.to_account_info().key.as_ref(), &[nonce]];
        //let signer = &[&seeds[..]];
        //let mut remaining_accounts: &[AccountInfo] = &[ctx.accounts.factory.to_account_info()];
        //let cpi_program = ctx.accounts.exchange_program.clone();
        //let cpi_accounts = Create::try_accounts(
        //    ctx.accounts.exchange.key,
        //    &mut remaining_accounts,
        //    &[],
        //)?;
        //let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        //exchange::cpi::create(cpi_ctx, factory.key(), token_a, token_b, token_c)?;
        exchange::cpi::create(ctx.accounts.into(), token_a, token_b, token_c, fee)?;
        Ok(())
    }

    pub fn get_exchange(_ctx: Context<GetExchange>, _token: Pubkey) -> ProgramResult {
        Ok(())
    }

    pub fn get_token(_ctx: Context<GetToken>, _token: Pubkey) -> ProgramResult {
        Ok(())
    }

    pub fn get_token_with_id(_ctx: Context<GetTokenWithId>, _token: Pubkey) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub factory: Account<'info, FactoryData>,
    pub authority: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CreateExchange<'info> {
    pub authority: Signer<'info>,
    #[account(zero)]
    pub exchange: Account<'info, ExchangeData>,
    #[account(mut)]
    pub factory: Account<'info, FactoryData>,
    pub exchange_program: Program<'info, Exchange>,
    pub token_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub exchange_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub exchange_b: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct GetExchange<'info> {
    pub factory: Account<'info, FactoryData>,
}

#[derive(Accounts)]
pub struct GetToken<'info> {
    pub factory: Account<'info, FactoryData>,
}

#[derive(Accounts)]
pub struct GetTokenWithId<'info> {
    pub factory: Account<'info, FactoryData>,
}

impl<'a, 'b, 'c, 'd, 'info> From<&mut CreateExchange<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Create<'info>>
{
    fn from(accounts: &mut CreateExchange<'info>) -> CpiContext<'a, 'b, 'c, 'info, Create<'info>> {
        let cpi_accounts = Create {
            factory: accounts.factory.to_account_info().clone(),
            exchange: accounts.exchange.to_account_info().clone().into(),
            token_program: accounts.token_program.to_account_info().clone(),
            exchange_a: accounts.exchange_a.to_account_info().clone(),
            exchange_b: accounts.exchange_b.to_account_info().clone(),
        };
        CpiContext::new(accounts.exchange_program.to_account_info().clone(), cpi_accounts)
    }
}

#[account]
pub struct FactoryData {
    pub exchange_template: Pubkey,
    pub token_count: u64,
}

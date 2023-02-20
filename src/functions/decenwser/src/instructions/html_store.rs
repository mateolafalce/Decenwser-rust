/*
    -Save html content on the web. This saving is consecutive and only 
    the private key of the authority can sign the transactions.
*/

use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn html_store(
    ctx: Context<HtmlStore>,
    content: String
) -> Result<()> {
    require!(ctx.accounts.main_account.len <= 9984, ErrorCode::TooLong);
    require!(ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), ErrorCode::AuthorityError);
    let (_pda, bump) = Pubkey::find_program_address(&[b"HTML", ctx.accounts.main_account.html.len().to_le_bytes().as_ref(), ctx.accounts.main_account.key().as_ref()], ctx.program_id);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let store: &mut Account<StoreAccount> = &mut ctx.accounts.store;
    let len: usize = main_account.html.len() + 1;
    store.content = content;
    store.bump_original = bump;
    main_account.html.push(len as u16);
    main_account.len += 2;
    Ok(())
}

#[derive(Accounts)]
pub struct HtmlStore<'info> {
    #[account(
        mut,
        seeds = [&anchor_lang::solana_program::hash::hash(main_account.web_name.as_bytes()).to_bytes()],
        bump = main_account.bump_original,
        realloc = 8 + main_account.len as usize + 2,
        realloc::payer = signer,
        realloc::zero = false,
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(init, seeds = 
        [
        b"HTML", 
        main_account.html.len().to_le_bytes().as_ref(), 
        main_account.key().as_ref()
        ], 
    bump, payer = signer, 
    space = StoreAccount::SIZE + 8
    )]
    pub store: Account<'info, StoreAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
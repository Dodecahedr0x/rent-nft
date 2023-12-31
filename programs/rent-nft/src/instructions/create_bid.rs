use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;

use crate::constants::*;
use crate::events::*;
use crate::state::*;

pub fn create_bid(ctx: Context<CreateBid>) -> Result<()> {
    msg!("Creating a bid account");

    let config = &mut ctx.accounts.config;
    msg!("{}", config.contest_window_size);
    let token_state = &mut ctx.accounts.token_state;
    let bid_state = &mut ctx.accounts.bid_state;

    bid_state.bidder = ctx.accounts.bidder.key();
    bid_state.token_state = token_state.key();
    bid_state.bidding_period = token_state.last_period;
    bid_state.last_update = Clock::get()?.unix_timestamp;
    bid_state
        .bids_window
        .resize(config.contest_window_size as usize, 0);

    emit!(CreatedBidState {
        bidder: ctx.accounts.bidder.key(),
        mint: token_state.token_mint.key(),
        collection: config.collection.key()
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CreateBid<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Delegatable creation
    pub bidder: UncheckedAccount<'info>,

    /// CHECK: Seeded authority
    #[account(
        mut,
        seeds = [
            &config.collection.to_bytes(),
            COLLECTION_AUTHORITY_SEED.as_bytes(),
        ],
        bump,
    )]
    pub collection_authority: UncheckedAccount<'info>,

    /// The config
    #[account(
        seeds = [
            &config.collection.to_bytes(),
        ],
        bump,
    )]
    pub config: Box<Account<'info, CollectionConfig>>,

    /// The state for the token assessement
    #[account(
        seeds = [
            &config.collection.to_bytes(),
            &token_state.token_mint.key().to_bytes()
        ],
        bump,
        has_one = config,
    )]
    pub token_state: Box<Account<'info, TokenState>>,

    #[account(
        init,
        payer = payer,
        space = BidState::len(config.contest_window_size),
        seeds = [
            &config.collection.to_bytes(),
            &token_state.token_mint.key().to_bytes(),
            &bidder.key().to_bytes(),
        ],
        bump,
    )]
    pub bid_state: Box<Account<'info, BidState>>,

    /// Common Solana programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

use anchor_lang::prelude::*;
use pyth_sdk_solana::PriceFeed;

declare_id!("5WM4KkVsMm6TGnJTyLrSikxT8rszdhRGi9AGgAJZWvqR");

#[program]
pub mod banding {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let price: PriceFeed =
            pyth_sdk_solana::load_price_feed_from_account_info(&ctx.accounts.price_act)
                .expect("price account fail");
        let state = &mut ctx.accounts.state;
        state.price = price.get_price_unchecked().price;
        state.ema_price = price.get_ema_price_unchecked().price;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        init,
        seeds = [b"state"],
        space = 128,
        bump,
        payer = payer,
    )]
    state: Account<'info, State>,
    /// CHECK
    price_act: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(Default, Debug)]
pub struct State {
    price: i64,
    ema_price: i64,
}

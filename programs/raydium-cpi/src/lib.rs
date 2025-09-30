#![allow(deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;


declare_id!("EXZHGtUuunmGujBhM6WxhCbZEqyN9bsAnsvXTyg1rqF");
declare_program!(raydium_launchpad);

#[program]
pub mod raydium_cpi {
    use super::*;

    pub fn create<'info>(
        ctx: Context<'_, '_, '_, 'info, Create<'info>>,
        args: LaunchTokenArgs,
    ) -> Result<()> {
        create_handler(ctx, args)
    }


}

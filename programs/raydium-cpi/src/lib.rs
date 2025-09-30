pub mod constants;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("EXZHGtUuunmGujBhM6WxhCbZEqyN9bsAnsvXTyg1rqF");

#[program]
pub mod raydium_cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}

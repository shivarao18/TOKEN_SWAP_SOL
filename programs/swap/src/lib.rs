pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GmHoo2EavRdygZv3xf4sXFR9UHoHNw54EVpyY4E9jgU5");

#[program]
pub mod swap {
    use super::*; 

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        initialize::handler(ctx)
    }
}

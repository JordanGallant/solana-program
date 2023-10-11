use anchor_lang::prelude::*;

declare_id!("mEAKv8Y3JDS1xzWDe8xBZTvdSFXW4i7ddo8dBCQ8S87");

#[program]
pub mod solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

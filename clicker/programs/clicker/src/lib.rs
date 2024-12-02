use anchor_lang::prelude::*;

declare_id!("Fm2vdosCXPotWTF8YMvATwfqNNZFEaEAcuF8yjXHrxvu");

#[program]
pub mod clicker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
use anchor_lang::prelude::*;

pub mod vm;

declare_id!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");

#[program]
pub mod gameboy {
    use super::*;

    pub fn new_instance(ctx: Context<NewInstanceContext>) -> Result<()> {
        **ctx.accounts.metadata = Metadata {
            game: ctx.accounts.game.key(),
            player: ctx.accounts.signer.key(),
            ram: ctx.accounts.ram.key(),
            save: ctx.accounts.save.key(),
        };

        Ok(())
    }
}

#[derive(Accounts)]
pub struct NewInstanceContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = Metadata::MAX_SIZE
    )]
    pub metadata: Box<Account<'info, Metadata>>,

    #[account(owner = cartridge::ID)]
    /// CHECK: check owner with constraint
    pub game: AccountInfo<'info>,

    /// CHECK: YOLO
    #[account(zero, signer)]
    pub save: AccountInfo<'info>,

    /// CHECK: YOLO
    #[account(zero, signer)]
    pub ram: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Metadata {
    pub player: Pubkey,
    pub game: Pubkey,
    pub save: Pubkey,
    pub ram: Pubkey,
}

impl Metadata {
    pub const MAX_SIZE: usize = 8 + 32 + 32 + 32;
}

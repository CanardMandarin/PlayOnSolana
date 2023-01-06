use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod cartridge {
    use super::*;

    pub fn install(ctx: Context<InstallGameContext>) -> Result<()> {
        **ctx.accounts.metadata = Metadata {
            game: ctx.accounts.game.key(),
            owner: ctx.accounts.signer.key(),
            is_active: false,
            bump: *ctx.bumps.get("metadata").unwrap(),
        };

        Ok(())
    }

    pub fn upload(ctx: Context<UploadGameContext>, offset: u64, bytes: Vec<u8>) -> Result<()> {
        let mut game_data = ctx.accounts.game.data.borrow_mut();
        let mut offset_copy = offset as usize;

        for byte in bytes.iter() {
            game_data[offset_copy] = *byte;
            offset_copy += 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InstallGameContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        seeds = ["GAME".as_bytes(), game.key().as_ref()],
        bump,
        payer = signer,
        space = Metadata::MAX_SIZE
    )]
    pub metadata: Box<Account<'info, Metadata>>,

    /// CHECK: YOLO
    #[account(zero, signer)]
    pub game: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UploadGameContext<'info> {
    #[account()]
    pub signer: Signer<'info>,
    #[account(
        constraint = signer.key() == metadata.owner,
        seeds = ["GAME".as_bytes(), game.key().as_ref()],
        bump = metadata.bump
    )]
    pub metadata: Box<Account<'info, Metadata>>,

    /// CHECK: YOLO
    #[account(
        mut,
    )]
    pub game: AccountInfo<'info>,
}

#[account]
pub struct Metadata {
    pub game: Pubkey,
    pub owner: Pubkey,
    pub is_active: bool,
    pub bump: u8,
}

impl Metadata {
    pub const MAX_SIZE: usize = 8 + 32 + 32 + 8 + 1 + 1;
}

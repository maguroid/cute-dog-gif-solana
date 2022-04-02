use anchor_lang::prelude::*;

declare_id!("7BVjk7wREn5FBPQhN18Um3pi5fQsyGn7Bm9mcFjmFzB3");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            id: base_account.total_gifs,
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            votes: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, gif_id: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        // let gif_iter = base_account.gif_list.into_iter();
        let gif_iter = &mut base_account.gif_list.iter_mut();
        match gif_iter.find(|item| item.id == gif_id) {
            Some(gif) => gif.votes += 1,
            _ => (),
        }

        // gif_iter
        //     .find(|item| item.id == gif_id)
        //     .and_then(|gif| Some(gif.votes += 1));

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub id: u64,
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: u64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}

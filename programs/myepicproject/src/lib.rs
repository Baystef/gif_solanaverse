use anchor_lang::prelude::*;

declare_id!("BcTYTvCuAz27yUb4h6pDEhkhaTvoR1si4VPzEcFdDfS2");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        //Get reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // initialize total_gifs to 0
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        //Get reference to the account
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            id: (base_account.gif_list.len() + 1).try_into().unwrap(),
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            upvotes: 0,
        };
        // add it to the gif_list Vector
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    // Find a gif by its id and upvote it
    pub fn upvote_gif(ctx: Context<UpvoteGif>, gif_id: u32) -> Result<()> {
        //Get reference to the account
        let base_account = &mut ctx.accounts.base_account;
        let gif_list = &mut base_account.gif_list;
        let index = gif_list.iter().position(|gif| gif.id == gif_id).unwrap();
        base_account.gif_list[index].upvotes += 1;
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context
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
pub struct UpvoteGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}


#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    id: u32,
    pub gif_link: String,
    pub user_address: Pubkey,
    pub upvotes: u32,
}

// Tell Solana what we want to store on this account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}

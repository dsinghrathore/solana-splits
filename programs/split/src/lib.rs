use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
use percentage::Percentage;
use std::collections::BTreeMap;
use std::collections::HashMap;

declare_id!("4tzDAD5KLntPhT8t3gjqs85vsT5aguZTNCoeRvKkt5zr");

#[program]
pub mod split {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.splits_count = 0;

        Ok(())
    }

    pub fn new_split(ctx: Context<NewSplitContext>, split: Vec<Map>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let mut total_percentage = 0;
        let mut index = 0;

        for item in split.iter() {
            total_percentage = total_percentage + item.perc;
            index = index + 1;
        }

        assert_eq!(
            total_percentage, 100,
            "NEW SPLIT: total percentage should be 100"
        );

        let new_split_id = base_account.splits_count + 1;
        base_account.splits.insert(new_split_id, split);

        Ok(())
    }

    #[allow(unused_assignments)]
    pub fn send_sol<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SenderContext<'info>>,
        split_id: u64,
        amount: u64,
    ) -> ProgramResult {
        let split_ref = &ctx.accounts.base_account.splits[&split_id];
        let msg_sender = &mut ctx.accounts.msg_sender;
        let mut index = 0;

        for rc_account in ctx.remaining_accounts.iter() {
            for split_map in split_ref.iter() {
                if &split_map.pub_key == &rc_account.key() {
                    let split_percentage = Percentage::from(split_map.perc);
                    let split_amount = split_percentage.apply_to(amount);
                    let ix = anchor_lang::solana_program::system_instruction::transfer(
                        &msg_sender.key(),
                        &rc_account.key(),
                        split_amount,
                    );
                    anchor_lang::solana_program::program::invoke(
                        &ix,
                        &[
                            msg_sender.to_account_info(),
                            ctx.remaining_accounts[index].to_account_info(),
                        ],
                    );
                    index = index + 1;
                }
            }

            panic!("account address doesn't exist in splits info");
        }

        Ok(())
    }
}

#[state]
#[derive(Default, Copy)]
pub struct Map {
    pub pub_key: Pubkey,
    pub perc: u64,
}

#[account]
pub struct BaseAccount {
    pub splits_count: u64,
    pub splits: BTreeMap<u64, Vec<Map>>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// pub struct MapVec{
//     item: HashMap<u64, std::rc::Rc<Map>>
// }

#[derive(Accounts)]
pub struct NewSplitContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SenderContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub msg_sender: Signer<'info>,
}

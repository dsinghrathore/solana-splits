use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
use percentage::Percentage;
use std::any::Any;
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

    pub fn new_split(
        ctx: Context<NewSplitContext>,
        split_perc: Vec<u64>,
        split_keys: Vec<Pubkey>,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let mut total_percentage = 0;
        let mut index = 0;

        for item in split_perc.iter() {
            total_percentage = total_percentage + item;
            index = index + 1;
        }

        assert_eq!(
            total_percentage, 100,
            "NEW SPLIT: total percentage should be 100"
        );

        let new_split_id = base_account.splits_count + 1;
        // base_account.splits.insert(new_split_id, split);
        base_account.splits_count = new_split_id;
        base_account.splits_perc.push(split_perc);
        base_account.splits_keys.push(split_keys);

        Ok(())
    }

    #[allow(unused_assignments)]
    pub fn send_sol<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SenderContext<'info>>,
        split_id: u64,
        amount: u64,
    ) -> ProgramResult {
        let split_perc = &ctx.accounts.base_account.splits_perc[split_id as usize];
        let split_keys = &ctx.accounts.base_account.splits_keys[split_id as usize];
        let msg_sender = &mut ctx.accounts.msg_sender;
        let mut index = 0;

        for rc_account in ctx.remaining_accounts.iter() {
            if split_keys.contains(&rc_account.key()) {
                let split_percentage = Percentage::from(split_perc[index]);
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

            panic!("account address doesn't exist in splits info");
        }

        Ok(())
    }
}

#[account]
pub struct BaseAccount {
    pub splits_count: u64,
    pub splits_perc: Vec<Vec<u64>>,
    pub splits_keys: Vec<Vec<Pubkey>>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

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

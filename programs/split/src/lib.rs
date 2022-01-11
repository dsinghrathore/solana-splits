use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
use percentage::Percentage;

declare_id!("4tzDAD5KLntPhT8t3gjqs85vsT5aguZTNCoeRvKkt5zr");

#[program]
pub mod split {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        Ok(())
    }

    pub fn new_split(
        ctx: Context<NewSplitContext>,
        split_creator: Pubkey,
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

        // base_account.splits_perc.push(split_perc);
        // base_account.splits_keys.push(split_keys);
        let n_split = Split {
            splits_creator: split_creator,
            splits_percentage: split_perc,
            splits_keys: split_keys,
            payments: vec![],
        };

        base_account.splits.push(n_split);

        Ok(())
    }

    #[allow(unused_assignments)]
    pub fn send_sol<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SenderContext<'info>>,
        split_id: u64,
        amount: u64,
        // receivers: &[AccountInfo]
        // receivers: Vec<Account<'info, T>>
    ) -> ProgramResult {
        // let split_perc = &ctx.accounts.base_account.splits_perc[split_id as usize];
        // let split_keys = &ctx.accounts.base_account.splits_keys[split_id as usize];
        let current_split = &mut ctx.accounts.base_account.splits[split_id as usize];
        // let split_perc = &current_split.splits_percentage;
        // let split_keys = &current_split.splits_keys;
        let msg_sender = &mut ctx.accounts.msg_sender;
        // let mut index = 0;

        let n_payment = Payment {
            total_amount: amount,
            paid_to: vec![],
        };

        current_split.payments.push(n_payment);

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &msg_sender.key(),
            // &ctx.accounts.system_program.key(),
            &ctx.accounts.bank_account.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                msg_sender.to_account_info(),
                // ctx.accounts.system_program.to_account_info()
                ctx.accounts.bank_account.to_account_info(),
            ],
        );

        // for rc_account in ctx.remaining_accounts.iter() {
        // for rc_account in receivers.iter() {
        //     if split_keys.contains(&rc_account.key()) {
        //         if rc_account.key() != msg_sender.key() {
        //             let split_percentage = Percentage::from(split_perc[index]);
        //             let split_amount = split_percentage.apply_to(amount);

        //             let ix = anchor_lang::solana_program::system_instruction::transfer(
        //                 &msg_sender.key(),
        //                 &rc_account.key(),
        //                 split_amount,
        //             );

        //             anchor_lang::solana_program::program::invoke(
        //                 &ix,
        //                 &[
        //                     msg_sender.to_account_info(),
        //                     receivers[index].to_account_info(),
        //                 ],
        //             );
        //         }

        //         index = index + 1;
        //     }
        // }

        Ok(())
    }

    pub fn withdraw(
        ctx: Context<WithdrawContext>,
        split_id: u64,
        payment_id: u64,
    ) -> ProgramResult {
        let current_split = &mut ctx.accounts.base_account.splits[split_id as usize];
        let current_payment = &mut current_split.payments[payment_id as usize];
        let split_percentages = &current_split.splits_percentage;
        let index = &mut 0;

        if !current_payment.paid_to.contains(&ctx.accounts.msg_sender.key()) {
            for c_key in current_split.splits_keys.iter() {
                if c_key == &ctx.accounts.msg_sender.key() {
                    let split_percentage = split_percentages[*index as usize];
                    let n_split_percentage = Percentage::from(split_percentage);
                    let split_amount = n_split_percentage.apply_to(current_payment.total_amount);
    
                    let ix = anchor_lang::solana_program::system_instruction::transfer(
                        // &ctx.accounts.system_program.key(),
                        &ctx.accounts.bank_account.key(),
                        &ctx.accounts.msg_sender.key(),
                        split_amount,
                    );
            
                    anchor_lang::solana_program::program::invoke(
                        &ix,
                        &[
                            // ctx.accounts.system_program.to_account_info(),
                            ctx.accounts.bank_account.to_account_info(),
                            ctx.accounts.msg_sender.to_account_info(),
                        ],
                    );
                }
            }    
        }

        Ok(())
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Split {
    pub splits_creator: Pubkey,
    pub splits_percentage: Vec<u64>,
    pub splits_keys: Vec<Pubkey>,
    pub payments: Vec<Payment>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Payment {
    pub total_amount: u64,
    pub paid_to: Vec<Pubkey>,
}

#[account]
pub struct BaseAccount {
    pub splits: Vec<Split>,
}

// #[account]
// pub struct BankAccount {
//     pub balance: u64,
// }

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
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub bank_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub msg_sender: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub bank_account: AccountInfo<'info>,
}

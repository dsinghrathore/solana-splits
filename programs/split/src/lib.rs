use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
use percentage::Percentage;
use anchor_lang::prelude::{Key, Signer};

#[derive(Debug)]
enum PDA {
    Pubkey,
}

declare_id!("4tzDAD5KLntPhT8t3gjqs85vsT5aguZTNCoeRvKkt5zr");

#[program]
pub mod split {
    use super::*;

    #[allow(unused_variables)]
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

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

        let n_split = Split {
            splits_creator: ctx.accounts.user.key(),
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
        amount: u64
    ) -> ProgramResult {
        let current_split = &mut ctx.accounts.base_account.splits[split_id as usize];
        let msg_sender = &mut ctx.accounts.user;

        let n_payment = Payment {
            total_amount: amount,
            paid_to: vec![],
        };

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &msg_sender.key(),
            &ctx.accounts.pda_account.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                msg_sender.to_account_info(),
                ctx.accounts.pda_account.to_account_info()
            ],
        )?;

        current_split.payments.push(n_payment);

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

        if !current_payment
            .paid_to
            .contains(&ctx.accounts.msg_sender.key())
        {
            for c_key in current_split.splits_keys.iter() {
                if c_key == &ctx.accounts.msg_sender.key() {
                    let split_percentage = split_percentages[*index as usize];
                    let n_split_percentage = Percentage::from(split_percentage);
                    let split_amount = n_split_percentage.apply_to(current_payment.total_amount);

                    let ix = anchor_lang::solana_program::system_instruction::transfer(
                        &ctx.accounts.pda_account.key(),
                        &ctx.accounts.receiver.key(),
                        split_amount,
                    );

                    anchor_lang::solana_program::program::invoke_signed(
                        &ix,
                        &[
                            ctx.accounts.pda_account.to_account_info(),
                            ctx.accounts.receiver.to_account_info(),
                            ctx.accounts.system_program.to_account_info()
                        ],
                        &[&[b"test", &[254]]]
                    )?;

                    current_payment.paid_to.push(ctx.accounts.receiver.key());
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
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SenderContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub pda_account: SystemAccount<'info>
}

#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub msg_sender: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub pda_account: SystemAccount<'info>,
    pub receiver: AccountInfo<'info>
}

use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
use anchor_lang::prelude::{Key, Signer};
use percentage::Percentage;

#[derive(Debug)]
enum PDA {
    Pubkey,
}

declare_id!("4tzDAD5KLntPhT8t3gjqs85vsT5aguZTNCoeRvKkt5zr");

#[program]
pub mod split {
    use super::*;

    #[allow(unused_variables)]
    pub fn initialize(ctx: Context<Initialize>, base_account_bump: u8) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        ctx.accounts.base_account.bump = base_account_bump;
        ctx.accounts.base_account.authority = *ctx.accounts.user.to_account_info().key;

        Ok(())
    }
    pub fn new_split(
        ctx: Context<NewSplitContext>,
        split_perc: Vec<u64>,
        split_keys: Vec<Pubkey>,
        split_account_bump: u8,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let mut total_percentage = 0;
        let mut index = 0;

        ctx.accounts.split_account.bump = split_account_bump;

        for item in split_perc.iter() {
            total_percentage = total_percentage + item;
            index = index + 1;
        }

        assert_eq!(
            total_percentage, 100,
            "NEW SPLIT: total percentage should be 100"
        );

        // let n_split = Split {
        //     splits_creator: ctx.accounts.user.key(),
        //     splits_percentage: split_perc,
        //     splits_keys: split_keys,
        //     payments: vec![],
        // };

        ctx.accounts.split_account.splits_creator = ctx.accounts.user.key();
        ctx.accounts.split_account.splits_percentage = split_perc;
        ctx.accounts.split_account.splits_keys = split_keys;
        ctx.accounts.split_account.payments = vec![];
        base_account.splits_nonce += 1;

        Ok(())
    }

    #[allow(unused_assignments)]
    pub fn send_sol<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SenderContext<'info>>,
        // split_id: u64,
        amount: u64,
    ) -> ProgramResult {
        let current_split = &mut ctx.accounts.split_account;
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
                ctx.accounts.pda_account.to_account_info(),
            ],
        )?;

        current_split.payments.push(n_payment);

        Ok(())
    }

    pub fn withdraw(
        ctx: Context<WithdrawContext>,
        // split_id: u64,
        payment_id: u64,
    ) -> ProgramResult {
        // let  current_split =  &mut ctx.accounts.split_account;
        // let current_payment =  &mut current_split.payments[payment_id as usize];
        // let split_percentages = &mut current_split.splits_percentage;
        let index = 0;

        if !ctx.accounts.split_account.payments[payment_id as usize]
            .paid_to
            .contains(&ctx.accounts.msg_sender.key())
        {
            for ind in 0..ctx.accounts.split_account.splits_keys.len() {
                if ctx.accounts.split_account.splits_keys[ind] == ctx.accounts.msg_sender.key() {
                    let split_percentage = ctx.accounts.split_account.splits_percentage[index];
                    let n_split_percentage = Percentage::from(split_percentage);
                    let split_amount = n_split_percentage.apply_to(
                        ctx.accounts.split_account.payments[payment_id as usize].total_amount,
                    );

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
                            ctx.accounts.system_program.to_account_info(),
                        ],
                        &[&[b"test", &[251]]],
                    )?;

                    ctx.accounts.split_account.payments[payment_id as usize]
                        .paid_to
                        .push(ctx.accounts.receiver.key());
                }
            }
        }

        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct SplitAccount {
    pub authority: Pubkey,
    pub bump: u8,
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
#[derive(Default)]
pub struct BaseAccount {
    pub splits_nonce: u64,
    pub bump: u8,
    pub authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(base_account_bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [
            b"test0".as_ref(),
            user.key().as_ref(),
        ],
        bump = base_account_bump,
        payer = user
    )]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(split_account_bump: u8)]
pub struct NewSplitContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(
        init,
        seeds = [
            b"test0".as_ref(),
            user.key().as_ref(),
        ],
        bump = split_account_bump,
        payer = user
    )]
    pub split_account: Account<'info, SplitAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SenderContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub pda_account: SystemAccount<'info>,
    #[account(mut)]
    pub split_account: Account<'info, SplitAccount>,
}

#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub msg_sender: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub pda_account: SystemAccount<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    // #[account(mut)]
    pub split_account: Account<'info, SplitAccount>,
}

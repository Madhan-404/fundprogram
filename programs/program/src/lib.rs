use anchor_lang::prelude::*;

declare_id!("FgGSSTdu28FdEGCQ77F5RyvWTRgC4qRGBhsMDH4pu2UW");

#[program]
pub mod fund {
    use super::*;

    pub fn create_fund(
        ctx: Context<CreateFund>,
        fund_name: String,
        fund_description: String,
        target_amount: u64,
    ) -> Result<()>{
        msg!("Fund Account Created");
        msg!("Fund Name: {}", fund_name);
        msg!("Fund Description: {}", fund_description);
        msg!("Target Amount: {}", target_amount);

        let fund = &mut ctx.accounts.fund;
        fund.user_id = ctx.accounts.initializer.key();
        fund.fund_name = fund_name;
        fund.fund_description = fund_description;
        fund.target_amount = target_amount;
        Ok(())
    }

    pub fn update_fund(
        ctx: Context<UpdateFund>,
        fund_name: String,
        fund_description: String,
        target_amount: u64,
    ) -> Result<()>{
        msg!("Fund Account Updated");
        msg!("Fund Name: {}", fund_name);
        msg!("Fund Description: {}", fund_description);
        msg!("Target Amount: {}", target_amount);

        let fund = &mut ctx.accounts.fund;
        fund.fund_description = fund_description;
        fund.target_amount = target_amount;
        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(fund_name:String, fund_description:String)]
pub struct CreateFund<'info> {
    #[account(
        init,
        seeds = [fund_name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer, 
        space = 8 + 32 + 4 + 4 + fund_name.len() + fund_description.len() + 8
    )]

    pub fund: Account<'info, FundAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(fund_name:String, fund_description:String)]
pub struct UpdateFund<'info> {
    #[account(
        mut,
        seeds = [fund_name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = 8 + 32 + 4 + 4 + fund_name.len() + fund_description.len() + 8,
        realloc::payer = initializer,
        realloc::zero = true,      //As the Account can be updated many times, the old data is not needed anymore.
    )]

    pub fund: Account<'info, FundAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut, close = user_id, has_one = user_id)]
    fund: Account<'info, FundAccountState>,
    #[account(mut)]
    user_id: Signer<'info>,
}

#[account]
pub struct FundAccountState {
    pub user_id: Pubkey, //32
    pub fund_name: String,  // 4 + len()
    pub fund_description: String,  // 4 + len()
    pub target_amount: u64, //8
}

   

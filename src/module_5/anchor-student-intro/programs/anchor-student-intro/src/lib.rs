use anchor_lang::prelude::*;

declare_id!("GKW5wWmYXw5ZYbBPCEc3ogEKZi5ToJqtNrijMoEnhBfC");

#[program]
pub mod anchor_student_intro {
    use super::*;

    pub fn update_student_intro(
        ctx: Context<UpdateStudentIntro>,
        name: String,
        message: String,
    ) -> Result<()> {
        msg!("Update student intro");
        msg!("Name: {name}");
        msg!("Message: {message}");

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.student = ctx.accounts.student.key();
        student_intro.name = name;
        student_intro.message = message;

        Ok(())
    }

    pub fn add_student_intro(
        ctx: Context<AddStudentIntro>,
        name: String,
        message: String,
    ) -> Result<()> {
        msg!("Add student intro");
        msg!("Name: {name}");
        msg!("Message: {message}");

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.student = ctx.accounts.student.key();
        student_intro.name = name;
        student_intro.message = message;

        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, seeds = [student.key().as_ref()], bump, close = student)]
    student_intro: Account<'info, StudentIntro>,
    #[account(mut)]
    student: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(name:String, message:String)]
pub struct UpdateStudentIntro<'info> {
    #[account(
        mut,
        seeds = [student.key().as_ref()],
        bump,
        realloc = 8 + 32 + 4 + name.len() + 4 + message.len(),
        realloc::payer = student,
        realloc::zero = false,
    )]
    pub student_intro: Account<'info, StudentIntro>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String, message:String)]
pub struct AddStudentIntro<'info> {
    #[account(
        init,
        seeds = [student.key().as_ref()],
        bump,
        payer = student,
        space = 8 + 32 + 4 + name.len() + 4 + message.len()
    )]
    pub student_intro: Account<'info, StudentIntro>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StudentIntro {
    pub student: Pubkey,
    pub name: String,
    pub message: String,
}

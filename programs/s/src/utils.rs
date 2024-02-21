
use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;

pub fn transfer_sol<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    amount: u64,
    signer: Option<&[&[&[u8]]]>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let transfer_instruction = anchor_lang::system_program::Transfer {
        from,
        to,
    };

    let cpi_context = match signer {
        Some(signer) => {
            CpiContext::new_with_signer(
                system_program,
                transfer_instruction,
                signer
            )
        }
        None => {
            CpiContext::new(
                system_program,
                transfer_instruction
            )
        }
    };

    anchor_lang::system_program::transfer(cpi_context, amount)?;

    Ok(())
}

pub fn transfer_token<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    amount: u64,
    signer: Option<&[&[&[u8]]]>,
    token_program: AccountInfo<'info>
) -> Result<()> {
    let transfer_instruction_account = Transfer {
        from,
        to,
        authority,
    };

    let cpi_context = match signer {
        Some(signer) => {
            CpiContext::new_with_signer(
                token_program,
                transfer_instruction_account,
                signer
            )
        }
        None => {
            CpiContext::new(
                token_program,
                transfer_instruction_account
            )
        }
    };

    anchor_spl::token::transfer(cpi_context, amount)?;

    Ok(())
}

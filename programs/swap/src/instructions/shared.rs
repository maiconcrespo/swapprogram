use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint,TokenAccount, TokenInterface,TransferChecked, transfer_checked}; // this can do that program can work witho older and new versions of the token program.
//asseguramos de importar todas las funciones que necesitamos para trabajar de "token_interface"
//creating a transfer function to help 
// the lifetime say that is hold onto them for as long as you need to hold onto.

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info,TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
)->Result<()>{
    //haremos una llamada en lowlevel of solana.
    let transfer_account_options = TransferChecked{
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_context = CpiContext::new(token_program.to_account_info(),transfer_account_options);
//este es nuestro programa de llamar todos los arugmentos que necesitamos para hacer la transferencia.
    transfer_checked(cpi_context,*amount,mint.decimals)
} 
// ahora tenemos una funcion par transferir tokens de una cuenta a otra.
//ahora podemos usar esta funcion en nuestro programa de swap.
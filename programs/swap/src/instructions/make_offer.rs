use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Offer, ANCHOR_DISCRIMINATOR};

use super::transfer_tokens;

#[derive(Accounts)]
#[instruction(id: u64)] //Agregamos que queremos aceder a la intruccion
pub struct MakeOffer<'info> {
    //Que tipo de cuenta necesitamos para cuando las personas quieran hacer una oferta.
    //Primero necesitamos la cuenta del proprietario que va a firmar la transaccion.
    #[account(mut)]
    pub maker: Signer<'info>,
    //Utilizaremos una Interface, porque puede que el token sea de un programa o de una extencion.
    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    //mutable porque vamos mover los tokens de una cuenta a otra.
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program

        )] //Especificamos donde se va a guardar la oferta.
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    //esta sera la PDA que salvaremos los detalles sobre lo que queremos en el cambio de tokens.
    #[account(
        init,
        payer=maker, //quien paga la creacion de la cuenta.
        space=ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds =[b"offer",maker.key().as_ref(),id.to_le_bytes().as_ref()],
        bump

    )]
    pub offer: Account<'info, Offer>,

    #[account(
        init,
        payer=maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program

    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_offered_tokens_to_vault(
    context: &Context<MakeOffer>,
    token_a_offered_amount: u64,
) -> Result<()> {
    transfer_tokens(
        &context.accounts.maker_token_account_a,
        &context.accounts.vault,
        &token_a_offered_amount,
        &context.accounts.token_mint_a,
        &context.accounts.maker,
        &context.accounts.token_program,
    )
}


pub fn save_offer(context: Context<MakeOffer>,id:u64,token_b_wanted_amount:u64)->Result<()>{
    context.accounts.offer.set_inner(Offer{
        id,
        maker: context.accounts.maker.key(),
        token_mint_a: context.accounts.token_mint_a.key(),
        token_mint_b: context.accounts.token_mint_b.key(),
        token_b_wanted_amount,
        bump: context.bumps.offer,
    });
    Ok(())
}
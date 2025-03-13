use anchor_lang::prelude::*;

#[account] //inicializamos la cuenta del programa
#[derive(InitSpace)] //inicializamos el espacio de la cuenta para no tener que hacer manualmente.
pub struct Offer{
    pub id: u64,
    pub maker:Pubkey,
    pub token_mint_a:Pubkey,
    pub token_mint_b:Pubkey,
    pub token_b_wanted_amount:u64,
    pub bump:u8,

}
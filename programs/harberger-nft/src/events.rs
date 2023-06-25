use anchor_lang::prelude::*;

#[event]
pub struct CollectionCreated {
    /// The config
    #[index]
    pub config: Pubkey,

    /// The collection mint
    pub collection: Pubkey,

    /// The tax mint
    pub tax_mint: Pubkey,
}

#[event]
pub struct TokenCreated {
    /// The config
    pub config: Pubkey,

    /// The created mint
    #[index]
    pub mint: Pubkey,

    /// The collection mint
    pub collection_mint: Pubkey,
}

#[event]
pub struct CreatedDepositAccount {
    /// The depositor
    pub depositor: Pubkey,

    /// The created mint
    #[index]
    pub mint: Pubkey,

    /// The collection mint
    pub collection_mint: Pubkey,
}

#[event]
pub struct BidUpdated {
    /// The collection mint
    pub collection_mint: Pubkey,

    /// The depositor
    pub depositor: Pubkey,

    /// The created mint
    #[index]
    pub mint: Pubkey,

    pub amount: i128,
}

#[event]
pub struct ClaimedToken {
    /// The collection mint
    #[index]
    pub collection_mint: Pubkey,

    /// The created mint
    #[index]
    pub mint: Pubkey,

    /// The new owner claiming the token
    pub claimant: Pubkey,
}

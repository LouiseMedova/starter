#![no_std]
use gstd::{prelude::*, ActorId};
use gmeta::{Metadata, InOut};
use gear_lib::non_fungible_token::{token::*, io::*};

#[derive(Encode, Decode, TypeInfo)]
pub struct InitNFT {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum NFTAction {
    Mint {
        to: ActorId,
        token_metadata: TokenMetadata
    },
    Burn {
        token_id: TokenId,
    },
    Transfer {
        token_id: TokenId,
        to: ActorId,
    },
    Approve {
        token_id: TokenId,
        to: ActorId,
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum NFTEvent {
    Mint(NFTTransfer),
    Burn(NFTTransfer),
    Transfer(NFTTransfer),
    Approve(NFTApproval)
}

pub struct NFTMetadata;

impl Metadata for NFTMetadata {
    type Init = InOut<InitNFT, ()>;
    type Handle = InOut<NFTAction, NFTEvent>;
    type State = NftIO;
    type Reply = ();
    type Signal = ();
    type Others = ();
}

#[derive(Encode, Decode, TypeInfo)]
pub struct NftIO {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub owner_by_id: Vec<(TokenId, ActorId)>,
    pub token_approvals: Vec<(TokenId, Vec<ActorId>)>,
    pub token_metadata_by_id: Vec<(TokenId, Option<TokenMetadata>)>,
    pub tokens_for_owner: Vec<(ActorId, Vec<TokenId>)>,
    pub token_id: TokenId,
    pub owner: ActorId,
}
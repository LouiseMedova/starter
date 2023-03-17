#![no_std]
use gstd::{prelude::*, msg, ActorId};
use gear_lib::non_fungible_token::{state::*, token::*, nft_core::*, io::*};
use gear_lib_derive::{NFTCore, NFTStateKeeper};
use nft_io::*;

static mut NFT: Option<MyNFT> = None;

#[derive(NFTCore, NFTStateKeeper, Default)]
pub struct MyNFT {
    #[NFTStateField]
    pub token: NFTState,
    pub token_id: TokenId,
    pub owner: ActorId,
}


#[no_mangle]
extern "C" fn init() {
    let init_msg: InitNFT = msg::load().expect("Inable to decode `InitNFT");
    let mut nft: MyNFT = Default::default();
    nft.token.name = init_msg.name;
    nft.token.symbol = init_msg.symbol;
    nft.token.base_uri = init_msg.base_uri;
    nft.owner = msg::source();
    unsafe { NFT = Some(nft) };
}

#[no_mangle]
extern "C" fn handle() {
    let msg: NFTAction = msg::load().expect("Inable to decode `NFTAction`");
    let nft = unsafe { NFT.as_mut().expect("The contract is no initialized") };
    match msg {
        NFTAction::Mint{to, token_metadata} => {
            reply(MyNFTCore::mint(nft, &to, token_metadata));
        },
        NFTAction::Burn{token_id} => {
            reply(nft.burn(token_id));
        }
        NFTAction::Transfer{to, token_id} => {
            reply(nft.transfer(&to, token_id));
        }, 
        NFTAction::Approve{to, token_id} => {
            reply(nft.approve(&to, token_id));
        }
    }
}
fn reply(payload: impl Encode) {
    msg::reply(payload, 0).expect("Error in sending a reply");
}

pub trait MyNFTCore: NFTCore {
    fn mint(&mut self, to: &ActorId, token_metadata: TokenMetadata) -> NFTTransfer;
}

impl MyNFTCore for MyNFT {
    fn mint(&mut self, to: &ActorId, token_metadata: TokenMetadata) -> NFTTransfer {
        assert_eq!(msg::source(), self.owner, "Only owner can mint new tokens");
        let token_id = self.token_id;
        self.token_id = self.token_id.saturating_add(TokenId::one());
        NFTCore::mint(self, to, token_id, Some(token_metadata))
    }
}

#[no_mangle]
extern "C" fn state() {
    let nft = unsafe { NFT.as_ref().expect("The contract is not initialized") };
    let state = NftIO {
        name: nft.token.name.clone(),
        symbol: nft.token.symbol.clone(),
        base_uri: nft.token.base_uri.clone(),
        owner_by_id: nft
            .token
            .owner_by_id
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect(),
        token_approvals: nft
            .token
            .token_approvals
            .iter()
            .map(|(key, value)| (*key, value.iter().copied().collect()))
            .collect(),
        token_metadata_by_id: nft
            .token
            .token_metadata_by_id
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect(),
        tokens_for_owner: nft
            .token
            .tokens_for_owner
            .iter()
            .map(|(key, value)| (*key, value.clone()))
            .collect(),
        token_id: nft.token_id,
        owner: nft.owner,
    };
    msg::reply(state, 0).expect("Unable to share the state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to share the metahash");
}
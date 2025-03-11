use std::{env, ops::Add, sync::Arc};

use ethers::{contract::abigen, middleware::SignerMiddleware, providers::{Http, Provider}, signers::{LocalWallet, Signer}, types::{self, Address, Chain, U256}};
use num_bigint::BigInt;
use tracing::info;
use std::str::FromStr;
use crate::{error::Error, prelude::Result};

//pub static provider: Lazy<Provider<Http>> = Lazy::new(setup_provider);

//fn setup_provider() -> Provider<Http> {
//    let rpc_url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL not set");
//    Provider::<Http>::try_from(rpc_url).expect("Canot create provider")
//}

abigen!(
    NftContract,
    "./erc721_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub async fn check_owner_has_nft(owner: &str, collection: &str, token_id: &BigInt) -> Result<()> {
    // make API call to chain provider
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
    let wallet =  private_key.parse::<LocalWallet>().map_err(|_| Error::Generic("Missing private key".to_owned()))?.with_chain_id(Chain::Mainnet);
    let rpc_url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL not set");
    let provider = Provider::<Http>::try_from(rpc_url).expect("Canot create provider");
    let client = SignerMiddleware::new(provider, wallet);
    
    info!("Collection Address: {:?}", Address::from_str(collection).unwrap());
    let erc721 = NftContract::new(Address::from_str(collection).unwrap(), Arc::new(&client));
    info!("---------------------------------------------");
    let name = erc721.name().call().await;
    info!("Token name: {}", name.unwrap());
    let token_id = U256::from_dec_str(&token_id.to_string()).unwrap();
    info!("tokenId: {}", token_id.to_string());
    let value = erc721.owner_of(token_id).call().await.map_err(|_| Error::Generic("Failed call".to_owned()))?;
    info!("NFT is owned by {}", value);
    if Address::from_str(owner).unwrap() == value {
        Ok(())
    } else {
        Err(Error::MissingNFT)
    }
}


use dotenv::dotenv;
use once_cell::sync::Lazy;
use tracing::debug;
use num_bigint::BigInt;
use std::str::FromStr;
use std::{env, sync::Arc};
use ethers::contract::abigen;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::{Address, Chain, U256};
use crate::{error::Error, prelude::Result};

static CLIENT: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> = Lazy::new(setup_client);

abigen!(
    NftContract,
    "./erc721_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

fn setup_client() -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    dotenv().ok();
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
    let wallet =  private_key.parse::<LocalWallet>().expect("Wrong format of private key").with_chain_id(Chain::Mainnet);
    let rpc_url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL not set");
    let provider = Provider::<Http>::try_from(rpc_url).expect("Canot create provider");
    Arc::new(SignerMiddleware::new(provider, wallet))
}

pub async fn check_owner_has_nft(owner: &str, collection: &str, token_id: &BigInt) -> Result<()> {
    let client = CLIENT.clone();
    let erc721 = NftContract::new(Address::from_str(collection).unwrap(), client);
    let name = erc721.name().call().await;
    debug!("Token name: {}", name.unwrap());
    let token_id = U256::from_dec_str(&token_id.to_string()).unwrap();
    debug!("tokenId: {}", token_id.to_string());
    let value = erc721.owner_of(token_id).call().await.map_err(|_| Error::Generic("Failed call".to_owned()))?;
    debug!("NFT is owned by {}", value);
    if Address::from_str(owner).unwrap() == value {
        Ok(())
    } else {
        Err(Error::MissingNFT)
    }
}


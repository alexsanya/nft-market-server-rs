use dotenv::dotenv;
use once_cell::sync::Lazy;
use tracing::debug;
use num_bigint::BigInt;
use std::str::FromStr;
use std::sync::Arc;
use ethers::contract::abigen;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::{Address, Chain, U256};
use crate::settings::SETTINGS;
use crate::{error::Error, prelude::Result};

static CLIENT: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> = Lazy::new(setup_client);

abigen!(
    NftContract,
    "./erc721_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    ERC20Contract,
    "./erc20_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

fn setup_client() -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    dotenv().ok();
    let settings = SETTINGS.clone();
    let private_key = settings.chain_data.private_key;
    let wallet =  private_key.parse::<LocalWallet>().expect("Wrong format of private key").with_chain_id(Chain::Mainnet);
    let rpc_url = settings.chain_data.rpc_url;
    let provider = Provider::<Http>::try_from(rpc_url).expect("Canot create provider");
    Arc::new(SignerMiddleware::new(provider, wallet))
}

pub async fn ensure_owner_has_nft(owner: &str, collection: &str, token_id: &BigInt) -> Result<()> {
    let client = CLIENT.clone();
    let contract_address = Address::from_str(collection).map_err(|_| Error::MissingNFT)?;
    let erc712 = NftContract::new(contract_address, client);
    let name = erc712.name().call().await.map_err(|_| Error::MissingNFT)?;
    debug!("NFT token name: {}", name);
    let token_id = U256::from_dec_str(&token_id.to_string()).map_err(|_| Error::MissingNFT)?;
    debug!("tokenId: {}", token_id.to_string());
    let value = erc712.owner_of(token_id).call().await.map_err(|_| Error::Generic("Failed call to provider".to_owned()))?;
    debug!("NFT is owned by {}", value);
    let owner_address = Address::from_str(owner).map_err(|_| Error::MissingNFT)?;
    if owner_address == value {
        Ok(())
    } else {
        Err(Error::MissingNFT)
    }
}

pub async fn esnure_erc712_allowance(owner: &str, collection: &str) -> Result<()> {
    let settings = SETTINGS.clone();
    let client = CLIENT.clone();
    let contract_address = Address::from_str(collection).map_err(|_| Error::MissingNFT)?;
    let owner_address = Address::from_str(owner).map_err(|_| Error::MissingNFT)?;
    let broker_address = Address::from_str(&settings.chain_data.broker_address).unwrap();
    let erc712 = NftContract::new(contract_address, client);
    let is_allowed = erc712.is_approved_for_all(owner_address, broker_address).call().await.map_err(|_| Error::Generic("Failed call to provider".to_owned()))?;
    if is_allowed {
        Ok(())
    } else {
        Err(Error::MissingNftAllowance)
    }
}

pub async fn ensure_bidder_has_tokens(bidder: &str, contract: &str, value: &BigInt) -> Result<()> {
    let client = CLIENT.clone();
    let contract_address = Address::from_str(contract).map_err(|_| Error::MissingTokens)?;
    let bidder_address = Address::from_str(bidder).map_err(|_| Error::MissingTokens)?;
    let erc20 =  ERC20Contract::new(contract_address, client);
    let name = erc20.name().call().await.map_err(|_| Error::MissingTokens)?;
    debug!("ERC20 token name: {}", name);
    let bidder_balance = erc20.balance_of(bidder_address).call().await.map_err(|_| Error::Generic("Failed call to provider".to_owned()))?;
    debug!("Bidder balance: {}", bidder_balance);
    if bidder_balance >= U256::from_dec_str(&value.to_string()).map_err(|_| Error::MissingTokens)? {
        Ok(())
    } else {
        Err(Error::MissingTokens)
    }
}

pub async fn ensure_erc20_allowance(bidder: &str, contract: &str, value: &BigInt) -> Result<()> {
    let settings = SETTINGS.clone();
    let client = CLIENT.clone();
    let contract_address = Address::from_str(contract).map_err(|_| Error::MissingERC20Allowance)?;
    let bidder_address = Address::from_str(bidder).map_err(|_| Error::MissingERC20Allowance)?;
    let broker_address = Address::from_str(&settings.chain_data.broker_address).unwrap();
    let erc20 =  ERC20Contract::new(contract_address, client);
    let bidder_allowance = erc20.allowance(bidder_address, broker_address).call().await.map_err(|_| Error::Generic("Failed to call provider".to_owned()))?;
    debug!("Bidder allowance: {}", bidder_allowance);
    if bidder_allowance >= U256::from_dec_str(&value.to_string()).map_err(|_| Error::MissingERC20Allowance)? {
        Ok(())
    } else {
        Err(Error::MissingERC20Allowance)
    }
}

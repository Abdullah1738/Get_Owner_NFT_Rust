use web3::contract::{Contract, Options};
use web3::ethabi::ParseLog;
use web3::futures::TryFutureExt;
use web3::types::{Address, H160, U256};
use std::env;
use std::fmt::Result;
use std::str::FromStr;
#[tokio::main]
async fn main() -> web3::Result<()> {
    //need to chnage this to a mainnet api - infura will be fine
    let transport = web3::transports::Http::new("https://rinkeby-light.eth.linkpool.io/")?;
    let web3 = web3::Web3::new(transport);

    //contract data. This will need to be changed when we deploy the contract / test contract etc
    let nft_addr = Address::from_str("0xbd784Ed8e296ABe68e7E46F088e845838FB2bE53").unwrap();
    let contract = Contract::from_json(web3.eth(), nft_addr, include_bytes!("abi.json")).unwrap();

    //chnage 12 here to the tokenID
    let result = contract.query("ownerOf", (U256::from((12))), None, Options::default(), None);
    let owner_of: Address = result.await.unwrap();
    println!("The owner of tokenID 12 is {:02X?}", owner_of);

    Ok(())
}
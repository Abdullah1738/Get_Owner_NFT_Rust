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
    let nft_addr = Address::from_str("0xFe7A13D085a099dB9daF50b803d2eB765c2E04df").unwrap();
    let contract = Contract::from_json(web3.eth(), nft_addr, include_bytes!("abi.json")).unwrap();


    //change 0xc... to the address of the sender
    let sender = Address::from_str("0xc0016f4AE265f7311B4B6991a7aafc4052A8d3E7").unwrap();

    //chnage 12 here to the tokenID
    let tokenID = U256::from(12);

    
    let result = contract.query("balanceOf", (sender,tokenID), None, Options::default(), None);
    let owner_of: U256 = result.await.unwrap();
    if owner_of == U256::from(1) {
     println!("Is owner");
    } else {
        println!("Not owner")
    }

    Ok(())
}
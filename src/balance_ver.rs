use std::str::FromStr;

use blake2::Blake2b;
use web3_rpc::web3::Web3;
use serde::Deserialize;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use web3_unit_converter::Unit;
use sha3::{ Digest, Keccak256 };
use ethereum_types::Address;
use anyhow::anyhow;
use crate::query::query_rpc;
pub async fn validate_address_balance(network: String, address: &str) -> anyhow::Result<bool> {
    if !address.starts_with("0x") {
        return Err(anyhow!("Invalid address"));
    }

    let rpc_url = match query_rpc(network) {
        Ok(res) => {
            // println!("Successfully obtained RPC URL: {}", res);
            res
        }
        Err(e) => {
            println!("Error finding network: {}", e);
            return Err(anyhow::anyhow!("Network not found"));
        }
    };

    let public_node_url = &rpc_url;

    let rpc = Web3::new(public_node_url.to_string());
    let r = rpc.eth_get_balance(&address, None).await?;
    println!("{:#?}", r);
    match r.result {
        Some(balance_wei) => {
            // Decode gas price from hexadecimal to bytes
            let temp = &balance_wei.trim_start_matches("0x");
            let t = u128::from_str_radix(temp, 16)?;
            let balance_eth = Unit::Wei(&t.to_string()).to_eth_str().unwrap();

            let balance: f64 = f64::from_str(balance_eth.as_str())?;
            println!("Balance is: {:?}", balance);
            if balance > 0.0 {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        None => {
            println!("Balance Invalid");
           
            return Ok(false);
        }
    }

    
}


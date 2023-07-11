use std::str::FromStr;

use blake2::Blake2b;
use web3_rpc::web3::Web3;
use serde::Deserialize;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use web3_unit_converter::Unit;
use sha3::{ Digest, Keccak256 };
use ethereum_types::Address;
use crate::query;
pub async fn validate_address_balance(
    network: String,
    to_address: &str,
    from_address: &str
) -> anyhow::Result<bool> {

    #[derive(Debug)]
    #[derive(Deserialize)]
    struct ApiResponse {
        status: String,
        message: String,
        result: Vec<Account>,
    }

    #[derive(Deserialize)]
    #[derive(Debug)]
    struct ApiError {
        code: i32,
        message: String,
    }

    #[derive(Deserialize)]
    #[derive(Debug)]
    struct Account {
        account: String,
        balance: String,
    }

    //get url
    let rpc_url = match query::query_rpc(network) {
        Ok(res) => {
            // println!("Successfully obtained RPC URL: {}", res);
            res
        }
        Err(e) => {
            println!("Error finding network: {}", e);
            return Err(anyhow::anyhow!("Network not found"));
        }
    };
    
    let url = format!(
        "{}?module=account&action=balancemulti&address={},{}&tag=latest&apikey=U5U1Q3YXX6BMNJ5DJVDK4EBYRUVZS3HBZI",
        rpc_url,
        to_address,
        from_address
    );

    let res = reqwest::get(&url).await?.json::<ApiResponse>().await?;
    println!("{:#?}", res);
    
    if &res.result[0].balance == "0" || &res.result[1].balance == "0" {
        Ok(false)
    } else {
        let r1 = Unit::Wei(res.result[0].balance.as_str()).to_eth_str().unwrap();
        println!("Value is to_address: {}", r1);
        let r2 = Unit::Wei(res.result[1].balance.as_str()).to_eth_str().unwrap();
        println!("Value is to_address: {}", r2);
        Ok(true)
    }
}

// fn query_rpc(network_name: String) -> anyhow::Result<String> {
//     let base_rpc = "https://api-testnet.";
//     let rpc_url: Option<String> = match network_name.to_lowercase().as_str() {
//         "polygon" => Some(format!("{}polygonscan.com/api", base_rpc)),
//         "avalanche" => Some(format!("{}snowtrace.io/api", base_rpc)),
//         _ => None,
//     };
//     //https://api-testnet.snowtrace.io/
//     //https://api-testnet.polygonscan.com/
//     match rpc_url {
//         Some(url) => Ok(url),
//         None => Err(anyhow::anyhow!("Invalid network name")),
//     }
// }


// pub fn verify_polygon_mumbai_address(address: &str) -> bool {
//     // Check address length
//     if address.len() != 42 {
//         return false;
//     }

//     // Check address prefix
//     if !address.starts_with("0x") {
//         return false;
//     }

//     // Perform checksum validation
//     let address_without_prefix = &address[2..];
//     let address_bytes = hex::decode(address_without_prefix).unwrap();
//     let checksum = &address_bytes[address_bytes.len() - 20..];
//     let address_without_checksum = &address_bytes[..address_bytes.len() - 20];
//     let calculated_checksum = Keccak256::digest(address_without_checksum);
//     if calculated_checksum[..] != checksum[..] {
//         return false;
//     }

//     // Address is considered valid
//     true
// }

// pub fn verify_avalanche_fuji_address(address: &str) -> bool {
//     // Check address length
//     if address.len() != 44 {
//         return false;
//     }

//     // Check address prefix
//     if !address.starts_with("0x") {
//         return false;
//     }

//     // Perform checksum validation
//     let address_without_prefix = &address[2..];
//     let address_bytes = hex::decode(address_without_prefix).unwrap();
//     let checksum = &address_bytes[address_bytes.len() - 4..];
//     let address_without_checksum = &address_bytes[..address_bytes.len() - 4];
//     let calculated_checksum = Keccak256::digest(address_without_checksum);
//     if calculated_checksum[..4] != checksum[..] {
//         return false;
//     }

//     // Address is considered valid
//     true
// }


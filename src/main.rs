use web3_rpc::web3::Web3;
use serde::Deserialize;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use web3_unit_converter::Unit;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let rpc = Web3::new("https://avalanche-fuji-c-chain.publicnode.com".to_string());

    // /* Send Raw Transaction */
    // let raw_tx_hash = format!(
    //     "0x{}",
    //     "f8716c8505d21dba00825268945852231d8a00306a67dfb128aed50c1573411d60840bebc20086010203040506830150f6a0ab8f07bd7c23c3568293fc7ebb0bdbd2375e3f65ac81f7564324a93cb0d6ed69a0557e7a185c265409d453a2bceebbe92d157fc665acf2b020d02879d6eb594907"
    // );
    // let send_raw = rpc.eth_send_raw_transaction(raw_tx_hash.as_str()).await?;

    // let tx_hash = match send_raw.result {
    //     Some(res) => {
    //         println!("Result: {:?}", res);
    //         res
    //     }
    //     None => {
    //         println!("Send Failed");
    //         "0x".to_string()
    //     }
    // };

    // println!("Tx hash: {}", &tx_hash);

    // thread::sleep(time::Duration::from_secs(3));
    // /* Get Transaction Receipt */
    // let r = rpc.eth_get_transaction_receipt(tx_hash.as_str()).await?;
    // // let r = rpc.eth_get_transaction_receipt("0xd0e2b9b7cf185d24058a030b60b01d9df40788beafda881107cb010416ea95df").await?;

    // let block_hash = match r.result {
    //     Some(v) => {
    //         println!("Success, Block hash: {:#?}", v);
    //         v.block_hash
    //     }
    //     None => {
    //         println!("Fail to query block hash");
    //         "0x".to_string()
    //     }
    // };

    // let b_h = block_hash.as_str();
    // let eth_block = rpc.eth_get_block_by_hash(b_h, true).await?;

    // // println!("Block Num: {:#?}", &eth_block);

    // let mut block_confirmation: u128 = 0;

    // match eth_block.result {
    //     Some(block) => {
    //         // get current block of inside the  network
    //         thread::sleep(time::Duration::from_secs(3));
    //         let block_field = rpc.eth_block_number().await?;
    //         let current_block = match block_field.result {
    //             Some(b) => {
    //                 let block_trim = b.trim_start_matches("0x");
    //                 let current_b = u128::from_str_radix(block_trim, 16)?;
    //                 println!("Sucess, current block is: {}", &current_b);
    //                 current_b
    //             }
    //             None => {
    //                 println!("Fail");
    //                 0
    //             }
    //         };

    //         let block_num = block.number.trim_start_matches("0x");
    //         let result = u128::from_str_radix(block_num, 16)?;

    //         println!("Block num of that transaction: {}", &result);

    //         println!("Current block number {}", &current_block);
    //         block_confirmation = current_block - result;

    //         println!("Block confirmation (curr - block num): {}", block_confirmation);
    //     }
    //     None => {
    //         println!("Faild");
    //     }
    // }
    // if block_confirmation >= 2 {
    //     println!("Successfully confirmed block, total confirmed block: {}", block_confirmation);
    // } else {
    //     println!("Failed, confirmed block must greater than 2");
    // }

    let v_ad = validate_address(
        "AVALANCHE".to_owned(),
        "0xCF6F0d155989B11Ba3882e99c72f609f0C06e086",
        "0x5852231D8a00306A67DfB128AEd50c1573411d60"
    ).await?;

    match v_ad {
        true => println!("Address Valid"),
        false => println!("Address Not Valid"),
    }
    Ok(())
}

fn query_rpc(network_name: String) -> anyhow::Result<String> {
    let rpc_url: Option<String> = match network_name.to_lowercase().as_str() {
        "polygon" => Some(format!("https://api-testnet.{}/api?", "polygonscan.com")),
        "avalanche" => Some(format!("https://api-testnet.{}/api?", "snowtrace.io")),
        _ => None,
    };
    //https://api-testnet.snowtrace.io/
    //https://api-testnet.polygonscan.com/
    match rpc_url {
        Some(url) => Ok(url),
        None => Err(anyhow::anyhow!("Invalid network name")),
    }
}

async fn validate_address(network: String, to_address: &str, from_address: &str) -> anyhow::Result<bool> {
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

    // let rpc = Web3::new(rpc_url.to_string());
    let url = format!(
        "{}module=account&action=balancemulti&address={},{}&tag=latest&apikey=U5U1Q3YXX6BMNJ5DJVDK4EBYRUVZS3HBZI",
        rpc_url,
        to_address,
        from_address
    );

    let res = reqwest::get(&url).await?.json::<ApiResponse>().await?;
    println!("{:#?}", res);
    if (&res.result[0].balance == "0" || &res.result[1].balance == "0") {
        Ok(false)
    } else {
        let r1 = Unit::Wei(res.result[0].balance.as_str()).to_eth_str().unwrap();
        println!("Value is to_address: {}", r1);
        let r2 = Unit::Wei(res.result[1].balance.as_str()).to_eth_str().unwrap();
        println!("Value is to_address: {}", r2);
        Ok(true)
    }

    // println!("Response: {:#?}", &res.id);
    // let r = rpc.eth_get_balance(address, None).await?;
    // match res {
    //     result => {
    //         println!("Response: {}", &result.result);
    //     }
    // }

    // Ok(true)
}

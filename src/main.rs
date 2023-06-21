use web3_rpc::web3::Web3;
use std::{thread, time};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rpc = Web3::new("https://avalanche-fuji-c-chain.publicnode.com".to_string());

    /* Send Raw Transaction */
    let raw_tx_hash = format!(
        "0x{}",
        "f8716c8505d21dba00825268945852231d8a00306a67dfb128aed50c1573411d60840bebc20086010203040506830150f6a0ab8f07bd7c23c3568293fc7ebb0bdbd2375e3f65ac81f7564324a93cb0d6ed69a0557e7a185c265409d453a2bceebbe92d157fc665acf2b020d02879d6eb594907"
    );
    let send_raw = rpc.eth_send_raw_transaction(raw_tx_hash.as_str()).await?;

    let tx_hash = match send_raw.result {
        Some(res) => {
            println!("Result: {:?}", res);
            res
        }
        None => {
            println!("Send Failed");
            "0x".to_string()
        }
    };

    println!("Tx hash: {}", &tx_hash);
    
    thread::sleep(time::Duration::from_secs(3));
    /* Get Transaction Receipt */
    let r = rpc.eth_get_transaction_receipt(tx_hash.as_str()).await?;
    // let r = rpc.eth_get_transaction_receipt("0xd0e2b9b7cf185d24058a030b60b01d9df40788beafda881107cb010416ea95df").await?;

    let block_hash = match r.result {
        Some(v) => {
            println!("Success, Block hash: {:#?}", v);
            v.block_hash
        }
        None => {
            println!("Fail to query block hash");
            "0x".to_string()
        }
    };

    let b_h = block_hash.as_str();
    let eth_block = rpc.eth_get_block_by_hash(b_h, true).await?;

    // println!("Block Num: {:#?}", &eth_block);

    let mut block_confirmation: u128 = 0;

    match eth_block.result {
        Some(block) => {
            // get current block of inside the  network
            thread::sleep(time::Duration::from_secs(3));
            let block_field = rpc.eth_block_number().await?;
            let current_block = match block_field.result {
                Some(b) => {
                    let block_trim = b.trim_start_matches("0x");
                    let current_b = u128::from_str_radix(block_trim, 16)?;
                    println!("Sucess, current block is: {}", &current_b);
                    current_b
                }
                None => {
                    println!("Fail");
                    0
                }
            };

            let block_num = block.number.trim_start_matches("0x");
            let result = u128::from_str_radix(block_num, 16)?;

            println!("Block num of that transaction: {}", &result);

            println!("Current block number {}", &current_block);
            block_confirmation = current_block - result;

            println!("Block confirmation (curr - block num): {}", block_confirmation);
        }
        None => {
            println!("Faild");
        }
    }
    if block_confirmation >= 2 {
        println!("Successfully confirmed block, total confirmed block: {}", block_confirmation);
    } else {
        println!("Failed, confirmed block must greater than 2");
    }

    Ok(())
}

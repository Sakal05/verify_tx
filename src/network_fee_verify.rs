use anyhow::{Result, anyhow};
use crate::balance_ver;

pub async fn verify_account_balance(
    network: String,
    to_address: &str,
    from_address: &str
) -> Result<bool> {
    let _network = network.clone();
    
    let (to_balance, from_balance) = tokio::try_join!(
        balance_ver::validate_address_balance(_network.clone(), to_address),
        balance_ver::validate_address_balance(_network.clone(), from_address),
    )?;
    
    if to_balance && from_balance {
        Ok(true)
    } else if !to_balance {
        Err(anyhow!("Sender Balance is insufficient"))
    } else if !from_balance {
        Err(anyhow!("Receiver Balance is insufficient"))
    } else {
        Err(anyhow!("Balance is insufficient"))
    }
}

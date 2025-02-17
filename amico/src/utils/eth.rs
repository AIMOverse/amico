use alloy::primitives::Address;

pub fn check_eth_balance(address: &Address) -> anyhow::Result<f64> {
    println!("Checking ETH balance for address: {}", address);
    Ok(0.01)
}

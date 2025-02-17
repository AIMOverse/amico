use std::{thread, time::Duration};

use alloy::primitives::Address;

pub fn check_eth_balance(address: &Address) -> anyhow::Result<f64> {
    tracing::debug!("Checking ETH balance for address: {}", address);

    thread::sleep(Duration::from_millis(500));

    Ok(0.01)
}

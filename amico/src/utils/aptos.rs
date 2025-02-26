// use std::str::FromStr;

// use aptos_sdk::{
//     coin_client::CoinClient,
//     rest_client::Client,
//     types::{account_address::AccountAddress, LocalAccount},
// };
// use url::Url;

// pub async fn check_aptos_balance(address: AccountAddress) -> anyhow::Result<f64> {
//     tracing::debug!("Checking Aptos balance for address: {}", address);

//     let rest_client = Client::new(Url::from_str("https://api.devnet.aptoslabs.com")?);
//     let coin_client = CoinClient::new(&rest_client);

//     let balance = coin_client.get_account_balance(&address).await?;

//     Ok(balance as f64 / 100_000_000 as f64)
// }

// pub async fn transfer_apt(
//     from: &LocalAccount,
//     to: &AccountAddress,
//     amount: u64,
// ) -> anyhow::Result<()> {
//     tracing::debug!(
//         "Transferring {} APT from {} to {}",
//         amount,
//         from.address(),
//         to
//     );

//     Ok(())
// }

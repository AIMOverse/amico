use amico::resource::Resource;
use solana_client::nonblocking::rpc_client::RpcClient;
use std::sync::Arc;

pub type ClientResource = Resource<Arc<RpcClient>>;

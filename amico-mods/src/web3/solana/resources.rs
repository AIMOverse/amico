use amico::resource::Resource;
use solana_client::nonblocking::rpc_client::RpcClient;

pub type ClientResource = Resource<RpcClient>;

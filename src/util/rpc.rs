use anyhow::Result;
use bdk::{
    bitcoincore_rpc::{Auth, Client, RpcApi},
    blockchain::rpc::{Auth as rpcAuth, RpcConfig},
};

pub fn rpc_client() -> Result<Client> {
    let auth = Auth::UserPass("admin1".to_string(), "123".to_string());
    Client::new("http://127.0.0.1:18443/wallet/", auth).map_err(|e| e.into())
}

pub fn mine_a_block(client: &Client) {
    let address = client.get_new_address(None, None).unwrap();
    client.generate_to_address(1, &address).unwrap();
}

pub fn rpc_config(wallet_name: String) -> RpcConfig {
    RpcConfig {
        url: "http://127.0.0.1:18443".to_string(),
        auth: rpcAuth::UserPass {
            username: "admin1".to_string(),
            password: "123".to_string(),
        },
        network: bdk::bitcoin::Network::Regtest,
        wallet_name,
        sync_params: None,
    }
}

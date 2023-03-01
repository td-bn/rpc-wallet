mod util;

use bdk::{
    bitcoin::{secp256k1::rand, Amount},
    bitcoincore_rpc::RpcApi,
    keys::bip39::{Language, Mnemonic},
};
use bdk_wallet::{wallet::Wallet, keys::get_descriptors};

use util::rpc::{mine_a_block, rpc_client, rpc_config} ;

#[test]
fn rpc_connection() {
    let client = rpc_client().unwrap();
    assert!(client.get_blockchain_info().is_ok());
}

#[test]
fn sending_sats_to_bdk_wallet() {
    let client = rpc_client().unwrap();

    let config = rpc_config("bwallet".to_string());

    let (r,c) = get_descriptors(mnemonic(), None);
    let wallet = Wallet::new(r, Some(c), config).unwrap();
    let address = wallet.new_address();

    let info = client.get_blockchain_info().unwrap();
    eprintln!("{:#?}", info.blocks);

    let _txid = client.send_to_address(
        &address,
        Amount::from_sat(10000000),
        None,
        None,
        None,
        None,
        None,
        None,
    ).unwrap();
    mine_a_block(&client);

    wallet.sync().unwrap();

    let info = client.get_blockchain_info().unwrap();
    eprintln!("{:#?}", info.blocks);

    let balance = wallet.get_balance().unwrap();
    assert!(balance > 0, "Balance: {:#?}", balance);
}

fn mnemonic() -> Mnemonic {
    let mut rng = rand::thread_rng();
    Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap()
}

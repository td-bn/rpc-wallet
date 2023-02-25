mod util;

use bdk::{
    bitcoin::{secp256k1::rand, Amount},
    bitcoincore_rpc::RpcApi,
    keys::bip39::{Language, Mnemonic},
    wallet::AddressIndex, SyncOptions,
};
use bdk_wallet::wallet::new_wallet;

use util::rpc::{mine_a_block, rpc_client, rpc_blockchain} ;

#[test]
fn rpc_connection() {
    let client = rpc_client().unwrap();
    assert!(client.get_blockchain_info().is_ok());
}

#[test]
fn sending_sats_to_bdk_wallet() {
    let client = rpc_client().unwrap();

    let wallet = new_wallet(mnemonic(), Some("".to_string())).unwrap();
    let address = wallet.get_address(AddressIndex::New).unwrap().address;

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

    let blockchain = rpc_blockchain("bdk_wallet".to_string());
    wallet.sync(&blockchain, SyncOptions::default()).unwrap();

    let info = client.get_blockchain_info().unwrap();
    eprintln!("{:#?}", info.blocks);

    let balance = wallet.get_balance().unwrap();
    assert!(balance.confirmed > 0, "Balance: {:#?}", balance);
}

fn mnemonic() -> Mnemonic {
    let mut rng = rand::thread_rng();
    Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap()
}

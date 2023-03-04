use bdk::{keys::{DerivableKey, ExtendedKey}, bitcoincore_rpc::RpcApi, bitcoin::Amount};
use bdk_wallet::{keys::mnemonic, manager::Manager, util::rpc::{rpc_client, mine_a_block}};

#[test]
fn send_sats_to_managed_wallet() {
    let client = rpc_client().unwrap();
    let xkey: ExtendedKey = (mnemonic(), None).into_extended_key().unwrap();
    let xprv = xkey.into_xprv(bdk::bitcoin::Network::Regtest).unwrap();

    let mut manager = Manager::new(xprv);

    let wallet_name = "testwally".to_string();

    manager.add_wpkh_wallet(wallet_name.clone()).unwrap();
    let wallet = manager.load_wallet(wallet_name.clone()).unwrap();

    let address = wallet.new_address();

    wallet.sync().unwrap();
    let balance_before = wallet.get_balance().unwrap();

    let _txid = client
        .send_to_address(
            &address,
            Amount::from_sat(10000000),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
    mine_a_block(&client);

    wallet.sync().unwrap();
    let balance_after = wallet.get_balance().unwrap();
    assert!(
        balance_after > balance_before,
        "Balance after: {:#?} Balance before: {:#?}",
        balance_after,
        balance_before
    );
}

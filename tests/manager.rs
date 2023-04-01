use bdk::{keys::{DerivableKey, ExtendedKey}, bitcoincore_rpc::RpcApi, bitcoin::Amount, SignOptions};
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


#[test]
fn multisig_using_managed_wallet() {
    let client = rpc_client().unwrap();
    let xkey: ExtendedKey = (mnemonic(), None).into_extended_key().unwrap();
    let xprv = xkey.into_xprv(bdk::bitcoin::Network::Regtest).unwrap();
    let mut manager = Manager::new(xprv);

    let xkey: ExtendedKey = (mnemonic(), None).into_extended_key().unwrap();
    let xprv = xkey.into_xprv(bdk::bitcoin::Network::Regtest).unwrap();
    let mut manager_w2 = Manager::new(xprv);


    let wallet_name = "multisig1".to_string();
    let wallet_other = "multisig2".to_string();

    let xpub1 = manager.initiate_multi_sig_wallet(wallet_name.clone()).unwrap();
    let xpub2 = manager_w2.add_multi_sig_wallet(wallet_other.clone(), 2, &[xpub1]).unwrap();

    let _ = manager.add_multi_sig_wallet(wallet_name.clone(), 2, &[xpub2]).unwrap();
    let wallet = manager.load_wallet(wallet_name).unwrap();

    let address = wallet.new_address();

    println!("{}", address);
    let address = wallet.new_address();
    println!("{}", address);
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

    // Spend from multisig

    let balance_before = balance_after;
    let recipient = client.get_new_address(None, None).unwrap();
    let mut tx = wallet.raw_wallet().build_tx();
    tx.set_recipients(vec![(recipient.script_pubkey(), 500000)]);
    let (mut psbt, _) = tx.finish().unwrap();

    // Sign multisig TX
    wallet.raw_wallet()
        .sign(&mut psbt, SignOptions::default())
        .unwrap();
    let w2 = manager_w2.load_wallet(wallet_other).unwrap();
    w2.raw_wallet()
        .sign(&mut psbt, SignOptions::default())
        .unwrap();

    // Broadcast and mine
    let tx = psbt.extract_tx();
    wallet.broadcast(tx).unwrap();
    mine_a_block(&client);

    wallet.sync().unwrap();

    let balance_after = wallet.get_balance().unwrap();

    // Balance lower after spend
    assert!(balance_after < balance_before);
}

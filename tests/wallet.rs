mod util;

use std::str::FromStr;

use bdk::{
    bitcoin::{
        secp256k1::{rand, Secp256k1},
        util::bip32::{DerivationPath, ExtendedPubKey},
        Amount,
    },
    bitcoincore_rpc::RpcApi,
    keys::{
        bip39::{Language, Mnemonic},
        DerivableKey, ExtendedKey,
    },
    SignOptions,
};
use bdk_wallet::{keys::get_descriptors, wallet::RpcWallet};

use util::rpc::{mine_a_block, rpc_client, rpc_config};

#[test]
fn rpc_connection() {
    let client = rpc_client().unwrap();
    assert!(client.get_blockchain_info().is_ok());
}

#[test]
fn sending_sats_to_bdk_wallet() {
    let client = rpc_client().unwrap();

    let config = rpc_config("bwallet".to_string());

    let (r, c) = get_descriptors(mnemonic(), None);
    let wallet = RpcWallet::new(r, Some(c), config).unwrap();
    let address = wallet.new_address();

    let info = client.get_blockchain_info().unwrap();
    eprintln!("{:#?}", info.blocks);

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

    let info = client.get_blockchain_info().unwrap();
    eprintln!("{:#?}", info.blocks);

    let balance = wallet.get_balance().unwrap();
    assert!(balance > 0, "Balance: {:#?}", balance);
}

#[test]
fn multisig() {
    let client = rpc_client().unwrap();
    let secp = Secp256k1::new();

    // Generate XPRIV, XPUB for first wallet
    let path = "m/84'/0'/0'/0/0";
    let config1 = rpc_config("w111".to_string());
    let xkey: ExtendedKey = (mnemonic(), None).into_extended_key().unwrap();
    let xprv1 = xkey.into_xprv(bdk::bitcoin::Network::Regtest).unwrap();
    let s1 = xprv1
        .derive_priv(&secp, &DerivationPath::from_str(path).unwrap())
        .unwrap();
    let xpub1 = ExtendedPubKey::from_priv(&secp, &s1);

    // Generate XPRIV, XPUB for second wallet
    let config2 = rpc_config("w221".to_string());
    let xkey: ExtendedKey = (mnemonic(), None).into_extended_key().unwrap();
    let xprv2 = xkey.into_xprv(bdk::bitcoin::Network::Regtest).unwrap();
    let s2 = xprv2
        .derive_priv(&secp, &DerivationPath::from_str(path).unwrap())
        .unwrap();
    let xpub2 = ExtendedPubKey::from_priv(&secp, &s2);

    let path = "/84'/0'/0'/0/*";
    let desc = format!("wsh(multi(2,{}{},{}))", xprv1, path, xpub2);
    let w1 = RpcWallet::new(desc, None, config1).unwrap();
    let desc = format!("wsh(multi(2,{},{}{}))", xpub1, xprv2, path);
    let w2 = RpcWallet::new(desc, None, config2).unwrap();

    let address = w1.new_address();

    let balance_before = w1.get_balance().unwrap();

    // Send sats to multisig address
    let _txid = client
        .send_to_address(
            &address,
            Amount::from_sat(10_00_000_000),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
    mine_a_block(&client);

    // W1 and W2 should have same balance
    w1.sync().unwrap();
    w2.sync().unwrap();

    let balance_after = w1.get_balance().unwrap();

    // Balance higher after receiving sats
    assert!(balance_after > balance_before);

    let balance_before = balance_after;

    // Create TX
    let recipient = client.get_new_address(None, None).unwrap();
    let mut tx = w1.raw_wallet().build_tx();
    tx.set_recipients(vec![(recipient.script_pubkey(), 500000)]);
    let (mut psbt, _) = tx.finish().unwrap();

    // Sign multisig TX
    w1.raw_wallet()
        .sign(&mut psbt, SignOptions::default())
        .unwrap();
    w2.raw_wallet()
        .sign(&mut psbt, SignOptions::default())
        .unwrap();

    // Broadcast and mine
    let tx = psbt.extract_tx();
    w1.broadcast(tx).unwrap();
    mine_a_block(&client);

    w1.sync().unwrap();

    let balance_after = w1.get_balance().unwrap();

    // Balance lower after spend
    assert!(balance_after < balance_before);
}

fn mnemonic() -> Mnemonic {
    let mut rng = rand::thread_rng();
    Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap()
}

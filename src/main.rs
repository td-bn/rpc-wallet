use anyhow::Result;
use bdk::bitcoin::secp256k1::rand;
use bdk::keys::bip39::{Language, Mnemonic};

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let _mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();

    Ok(())
}

/*
 * TODO:
 *  - add rpc connection to Regtest
 *  - send some sats to BDK wallet
 *  - integration test for adding balance to wallet
 *  - multisig b/w two wallets
 *
 *  - any other interesting thing?
 */

use anyhow::Result;
use bdk::bitcoin::secp256k1::rand;
use bdk::keys::bip39::{Language, Mnemonic};
use bdk_wallet::wallet::new_wallet;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();

    let _wallet = new_wallet(mnemonic, None)?;

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

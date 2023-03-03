use std::str::FromStr;

use anyhow::{Ok, Result};
use bdk::{
    bitcoin::{Network, util::bip32::ExtendedPrivKey},
    database::MemoryDatabase,
    template::Bip84,
    Wallet, blockchain::ElectrumBlockchain, electrum_client::Client, SyncOptions, wallet::AddressIndex,
};

fn main() -> Result<()> {
    let wallet = wallet()?;
    let blockchain = blockchain();
    
    let sync_opts = SyncOptions::default();

    wallet.sync(&blockchain, sync_opts)?;
    let address = wallet.get_address(AddressIndex::New)?.address;

    println!("address: {}", address);
    println!("balance: {}", balance(wallet)?);

    Ok(())
}

fn balance(wallet: Wallet<MemoryDatabase>) -> Result<u64> {
    let balance = wallet.get_balance()?;
    println!("{:#?}", balance);
    Ok(balance.confirmed)
}
 
fn wallet() -> Result<Wallet<MemoryDatabase>> {
    let xprv = "tprv8ZgxMBicQKsPeEWNmZ7B945tnYNiF3sfg2d51WykrzmEEMUWR1g85dQk8rchUq8sdgMw2tA1KbWsCQgi99PQadDMTuJJZ7nChSTjiZuxGVD";

    Wallet::new(
        Bip84(ExtendedPrivKey::from_str(xprv)?, bdk::KeychainKind::External),
        None,
        Network::Testnet,
        MemoryDatabase::new(),
    )
    .map_err(|e| e.into())
}

fn blockchain() -> ElectrumBlockchain {
    let electrum_url = "ssl://electrum.blockstream.info:60002";
    ElectrumBlockchain::from(Client::new(electrum_url).unwrap())
}

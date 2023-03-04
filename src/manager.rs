use std::{collections::HashMap, str::FromStr};

use crate::{util::rpc::rpc_config, wallet::RpcWallet};
use anyhow::{anyhow, Ok, Result};
use bdk::bitcoin::{
    secp256k1::Secp256k1,
    util::bip32::{DerivationPath, ExtendedPrivKey},
};

pub struct Manager<'a> {
    xprv: ExtendedPrivKey,
    index: usize,
    wallets: HashMap<String, RpcWallet>,
    current_wallet: Option<&'a RpcWallet>,
}

impl<'a> Manager<'a> {
    pub fn new(xprv: ExtendedPrivKey) -> Self {
        Self {
            xprv,
            index: 0,
            wallets: HashMap::new(),
            current_wallet: None,
        }
    }

    // Add a wpkh wallet
    pub fn add_wpkh_wallet(&mut self, name: String) -> Result<()> {
        let existing = self.wallets.get(&name);
        if let Some(_) = existing {
            return Err(anyhow!("Wallet name already exists"));
        }

        let secp = Secp256k1::new();
        let base_path = format!("m/84'/1'/0'/{}", self.index);
        self.index += 1;
        let derivation_path = DerivationPath::from_str(&base_path)?;
        let derived_xprv = &self.xprv.derive_priv(&secp, &derivation_path).unwrap();

        let desc = format!("wpkh({derived_xprv})");

        let config = rpc_config(name.clone());
        let wallet = RpcWallet::new(desc, None, config)?;

        self.wallets.insert(name, wallet);
        Ok(())
    }

    // Returns a reference to current wallet
    pub fn load_wallet(&'a mut self, name: String) -> Result<&RpcWallet> {
        let wallet = self.wallets.get(&name);
        if let Some(w) = wallet {
            self.current_wallet = Some(w);
            return  Ok(w);
        }
        Err(anyhow!("No such wallet found"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{keys::mnemonic, manager::Manager};
    use bdk::{
        bitcoin::Network,
        keys::{DerivableKey, ExtendedKey},
    };

    #[test]
    fn create_manager() {
        let xkey: ExtendedKey = (mnemonic(), None).into_extended_key().unwrap();
        let xprv = xkey.into_xprv(bdk::bitcoin::Network::Regtest).unwrap();

        let _ = Manager::new(xprv);
    }

    #[test]
    fn add_wallet() {
        let xkey: ExtendedKey = (mnemonic(), None).into_extended_key().unwrap();
        let xprv = xkey.into_xprv(Network::Regtest).unwrap();
        let mut manager = Manager::new(xprv);

        let name = "testW";
        manager.add_wpkh_wallet(name.to_owned()).unwrap();
    }

    #[test]
    fn add_duplicate_wallet() {
        let xkey: ExtendedKey = (mnemonic(), None).into_extended_key().unwrap();
        let xprv = xkey.into_xprv(Network::Regtest).unwrap();
        let mut manager = Manager::new(xprv);

        let name = "testW";
        manager.add_wpkh_wallet(name.to_owned()).unwrap();
        // Adding duplicate wallet
        let e = manager.add_wpkh_wallet(name.to_owned());
        assert!(e.is_err());
    }
}

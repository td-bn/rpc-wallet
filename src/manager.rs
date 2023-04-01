use std::{collections::HashMap, str::FromStr};

use crate::{util::rpc::rpc_config, wallet::RpcWallet};
use anyhow::{anyhow, Ok, Result};
use bdk::bitcoin::{
    secp256k1::Secp256k1,
    util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
};

struct InitiatedMultisig {
    path: DerivationPath,
}

pub struct Manager<'a> {
    xprv: ExtendedPrivKey,
    index: usize,
    wallets: HashMap<String, RpcWallet>,
    current_wallet: Option<&'a RpcWallet>,
    initiated_multisigs: HashMap<String, InitiatedMultisig>,
}

impl<'a> Manager<'a> {
    pub fn new(xprv: ExtendedPrivKey) -> Self {
        Self {
            xprv,
            index: 0,
            wallets: HashMap::new(),
            current_wallet: None,
            initiated_multisigs: HashMap::new(),
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
            return Ok(w);
        }
        Err(anyhow!("No such wallet found"))
    }

    // Initiate a multisig wallet
    pub fn initiate_multi_sig_wallet(&mut self, name: String) -> Result<ExtendedPubKey> {
        let existing = self.wallets.get(&name);
        if let Some(_) = existing {
            return Err(anyhow!("Wallet name already exists"));
        }
        let existing = self.initiated_multisigs.get(&name);
        if let Some(_) = existing {
            return Err(anyhow!("Already initiated a multisig with same name"));
        }

        let secp = Secp256k1::new();
        let derivation_path = self.new_path();
        let derived_xprv = &self.xprv.derive_priv(&secp, &derivation_path).unwrap();
        let derived_xpub = ExtendedPubKey::from_priv(&secp, &derived_xprv);

        self.initiated_multisigs
            .insert(name.clone(), InitiatedMultisig { path: derivation_path });

        Ok(derived_xpub)
    }

    // Add a k-of-n multisig wallet whether initiated or not
    pub fn add_multi_sig_wallet(
        &mut self,
        name: String,
        k: usize,
        pubkeys: &[ExtendedPubKey],
    ) -> Result<ExtendedPubKey> {
        let existing = self.wallets.get(&name);
        if let Some(_) = existing {
            return Err(anyhow!("Wallet name already exists"));
        }

        let secp = Secp256k1::new();
        let existing = self.initiated_multisigs.get(&name);
        let mut der_path_exists = false;
        let derivation_path = if let Some(_) = existing {
            der_path_exists = true;
            self.initiated_multisigs.remove(&name).unwrap().path
        } else {
            self.new_path()
        };
        // let desc_priv = format!("{}/84'/1'/0'/0", self.xprv);
        let derived_xprv = &self.xprv.derive_priv(&secp, &derivation_path).unwrap();
        let derived_xpub = ExtendedPubKey::from_priv(&secp, &derived_xprv);

        // Test with 2 of 2
        let desc = if der_path_exists {
            format!("wsh(multi({},{}/*,{}))", k, pubkeys[0], derived_xprv)
        } else {
            format!("wsh(multi({},{},{}/*))", k, derived_xprv, pubkeys[0])
        };
        println!("DESC: {:#?}", desc);
        println!("PATH: {:#?}", derivation_path);

        let config = rpc_config(name.clone());
        let wallet = RpcWallet::new(desc, None, config)?;

        self.wallets.insert(name, wallet);
        Ok(derived_xpub)
    }

    fn new_path(&mut self) -> DerivationPath {
        let path = DerivationPath::from_str(&format!("m/84'/1'/0'/{}",self.index)).unwrap();
        self.index += 1;
        path
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

use std::str::FromStr;

use anyhow::Result;
use bdk::{
    bitcoin::{self, secp256k1::Secp256k1, util::bip32::DerivationPath},
    database::MemoryDatabase,
    keys::DerivableKey,
    keys::{bip39::Mnemonic, DescriptorKey, ExtendedKey},
    miniscript::Segwitv0,
};

pub(crate) type Wallet = bdk::Wallet<MemoryDatabase>;

// Generate a new in-memory BDK wallet using a mnemonic, and a password
pub fn new_wallet(mnemonic: Mnemonic, password: Option<String>) -> Result<Wallet> {
    let (r, c) = get_descriptor(mnemonic, password);
    let wallet = bdk::Wallet::new(
        &r,
        Some(&c),
        bitcoin::Network::Regtest,
        MemoryDatabase::new(),
    )?;
    Ok(wallet)
}

// generate new descriptor and return descriptors for (receive, change)
fn get_descriptor(mnemonic: Mnemonic, password: Option<String>) -> (String, String) {
    let secp = Secp256k1::new();

    let xkey: ExtendedKey = (mnemonic, password).into_extended_key().unwrap();

    let xprv = xkey.into_xprv(bitcoin::Network::Regtest).unwrap();

    let mut keys = Vec::new();

    for path in ["m/84h/1h/0h/0", "m/84h/1h/0h/1"] {
        let deriv_path = DerivationPath::from_str(path).unwrap();
        let derived_xprv = &xprv.derive_priv(&secp, &deriv_path).unwrap();

        let origin = (xprv.fingerprint(&secp), deriv_path);
        let derived_xprv_desc_key: DescriptorKey<Segwitv0> = derived_xprv
            .into_descriptor_key(Some(origin), DerivationPath::default())
            .unwrap();

        // wrap the descriptor in a wpkh() string
        if let DescriptorKey::Secret(key, _, _) = derived_xprv_desc_key {
            keys.push(format!("wpkh({})", key.to_string()));
        }
    }
    (keys[0].clone(), keys[1].clone())
}

#[test]
fn test_desrciptor() {
    use bdk::bitcoin::secp256k1::rand;
    use bdk::keys::bip39::Language;

    let mut rng = rand::thread_rng();
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();

    let (r, c) = get_descriptor(mnemonic, None);
    assert!(r.starts_with("wpkh"));
    assert!(c.starts_with("wpkh"));
}

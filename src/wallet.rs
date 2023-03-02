use anyhow::Result;
use bdk::{
    bitcoin::{self, Address, Transaction},
    blockchain::{ConfigurableBlockchain, RpcBlockchain, RpcConfig, Blockchain},
    database::MemoryDatabase,
    wallet::AddressIndex,
    SyncOptions,
};

pub struct Wallet {
    wallet: bdk::Wallet<MemoryDatabase>,
    // TODO: Make this more generic?
    blockchain: RpcBlockchain,
}

impl Wallet {
    // Generate a new in-memory BDK wallet using a mnemonic, and a password
    pub fn new(r: String, c: Option<String>, config: RpcConfig) -> Result<Self> {
        let wallet = bdk::Wallet::new(
            &r,
            c.as_ref(),
            bitcoin::Network::Regtest,
            MemoryDatabase::new(),
        )?;
        let blockchain = RpcBlockchain::from_config(&config)?;
        Ok(Self { wallet, blockchain })
    }

    // Returns a new address using a new AddressIndex
    pub fn new_address(&self) -> Address {
        self.wallet.get_address(AddressIndex::New).unwrap().address
    }

    // Sync with the blockchain
    pub fn sync(&self) -> Result<()> {
        self.wallet
            .sync(&self.blockchain, SyncOptions::default())
            .map_err(|e| e.into())
    }

    // Returns confirmed balance of the BDK wallet
    pub fn get_balance(&self) -> Result<u64> {
        let bal = self.wallet.get_balance()?.confirmed;
        Ok(bal)
    }

    pub fn broadcast(&self, tx: Transaction) -> Result<()> {
        self.blockchain.broadcast(&tx)
            .map_err( |e| e.into())
    }

    // Returns bdk wallet instance
    pub fn raw_wallet(&self) -> &bdk::Wallet<MemoryDatabase> {
        &self.wallet
    }
}

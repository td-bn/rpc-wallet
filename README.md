# Simple BDK wallet implementation

This is a simple educational implementation of a BDK RpcWallet along with some tools to easily manage multisigs.

Uses MemoryDatabase for all wallets. All wallets use an RpcBlockchain to interact with the Bitcoin network.

Features:
 - sending and receiving BTC using simple wallets
 - create new multisig wallets with ease
 - sending and receiving BTC using multisig wallets


## How to run?

Because this is an RPC wallet you need to set up a RPC client in the Regtest env.
I use Nigiri. Makes testing easy.

Just run tests.
```bash
cargo test
```

Sometimes the RPC client throws the error. This is a known issue: https://github.com/bitcoindevkit/bdk/issues/749
```bash
Rpc(JsonRpc(Transport(SocketError(Os { code: 35, kind: WouldBlock, message: "Resource temporarily unavailable" }))))
```
This happens only when running all tests.

This error is not raise when running individual tests.
Individual tests can be run by just using the test name(or part of it) e.g.
```bash
cargo test multisig
# OR
cargo test multi 
```

## Concepts

This main structs in this repo are:

### RpcWallet
Simple descriptor based wallet that is associated with an `RpcBlockchain`.
Uses the `RpcBlockchain` to communicate with the BTC Network.

Exposes the following behaviour:
 - `new` create new wallet
 - `new_address` wrapper around BDK function
 - `sync` wrapper around BDK function, uses default sync options to sync with associated `RpcBlockchain`
 - `broadcast` wrapper around BDK function, uses associated `RpcBlockchain`to broadcast tx
 - `get_balance` wrapper around BDK function, returns confirmed balance
 - `raw_wallet` returns raw BDK wallet, to create txs, sign etc

### Manager
Manages key generation and wallets. Exposes simple behaviour.

Wallets are associated with names. Names are unique. Work with wallets using names and RpcWallet struct.

Exposes the following behaviour:
- `new` Takes an XPRV key to create the different wallets
- `add_wpkh_wallet` Generate a new BDK wallet for wpkh addresses
- `initiate_multi_sig_wallet` Generate keyspair for multisig wallets, return XPUB
- `add_multi_sig_wallet` Generate a new multisig wallet given some XPUBs and threshold, return XPUB
- `load_wallet` returns an instance of RpcWallet to work with

## Improvements
- export/import 
- use some DB instead of MemoryDatabase


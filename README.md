# Simple BDK wallet

This is a simple BDK wallet. This was an exercise in understanding and learning how to use BDK.

## How to run?

For running the tests, you need to set up a RPC client in the Regtest env.
I use Nigiri. Makes testing easy.

Just run tests.
```bash
cargo test
```

Sometimes the RPC client throws the error: 
```bash
Rpc(JsonRpc(Transport(SocketError(Os { code: 35, kind: WouldBlock, message: "Resource temporarily unavailable" }))))
```
in which case the issue was fixed by using a new wallet name in the test.

Individual tests can be run by just using the test name(or part of it) e.g.
```bash
cargo test multisig
# OR
cargo test multi 
```


## Improvements

- random wallet names to prevent RPC error
- better wallet design for multisig txs


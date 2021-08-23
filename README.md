# Mount Doom

This is a smart contract for people to burn their tokens on the Secret Network.
It has no admin functionality so it is completely decentralized and only has two functions:
1. Set a viewing key for a snip20 token for this smart contract (Buttcoin, sSCRT etc)
2. Query the viewing key for this smart contract

## How it works

* Any user can send any token on the Secret Network to this smart contract (native/SNIP-20).
* Any user can set a viewing key for any SNIP-20 token for this smart contract (the viewing key is set on contract initialization and cannot be changed).
* Any user can query this smart contract for the viewing key.
* Any user can then use that viewing key to check the balance and transactions.


### version 1.0.0 is currently available on Stargaze Mainnet 
You can interact with it using our dedicated UI here: [pegasus.zone](https://pegasus.zone/). 
The contract interaction can be found on mintscan: https://www.mintscan.io/stargaze/wasm/code/14 

# Peer-2-Peer NFT trade contract


This contract allows you to offer any NFT('s) you own for an NFT or multiple that someone else owns.

Anyone can create an Offer by calling `CreateOffer`, which contains the NFT's you would like to offer, The NFT's you would like to recieve, the peer which ownes the requested NFT's and an optional expiry date. If no expiry date is provided, the minimum will be used.

When a offer is pending the following executions can be performed:
- The Creator can revoke it using `RemoveOffer` 
- The peer can reject it using `RejectOffer`
- The peer can accept it using `AcceptOffer`
- The contract admin can remove expired offers using `RemoveStaleOffer`


### Authorizing Trade Contract
In order for the contract to create a Offer, the owner of the offered NFT's needs to approve the contract to transfer those NFT's (see cw721-base [approve message](https://github.com/CosmWasm/cw-nfts/blob/4e26419bb02f4b871fda487964a80bd419207428/contracts/cw721-base/src/execute.rs#L50))

In order for the peer to accept a pending offer, the peer needs to approve the contract first to transfer the wanted NFT's.

This needs to be done in the frontend, and we recommend grouping those transactions together with the execute message sent to this contract.
**Approval transactions go first in the list!**


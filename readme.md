# How this contract works

## Motivation
If you want to swap some juno for some osmosis, you'll have to first send 1 ibc transfer transaction from juno to osmosis. Once the ibc transfer transaction is successful, your osmosis account receives the ibc juno token and you can execute swap transaction on osmosis chain.

This contract is deployed on osmosis. It enable swaping with using only 1 ibc transaction and you'll receive osmosis on your osmosis account as if you just do the 2 steps described above.

Let's called this feature `instant ibc swap`.

## Concept

This contract is esenstially an ica module with its account being kinda like the ica account. However, the functionality is much more flexible and versatile than ica :

1. This contract account is not only controlled by only one remote account. It can be controlled by any remote account (weather that it's a contract account or just a regular account). But there's a catch, a remote account can only controll just the fund it sends to this contract account. For example, If a remote contract from juno chain send 50 ibc juno to this contract account with [FundMsg](https://github.com/notional-labs/gamm-bounty/blob/8c0682cfbb741066de7b78b8aff7a3b55866a1fb/contracts/ibc-gamm-osmosis/src/msg.rs#L21), that remote contract can only use a total of 50 juno to `instant ibc swap`.


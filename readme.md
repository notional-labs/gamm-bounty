# How this contract works

## Motivation
If you want to swap some juno for some osmosis, you'll have to first send 1 ibc transfer transaction from juno to osmosis. Once the ibc transfer transaction is successful, your osmosis account receives the ibc juno token and you can execute swap transaction on osmosis chain.

This contract is deployed on osmosis. It enable swaping with using only 1 ibc transaction and you'll receive osmosis on your osmosis account as if you just do the 2 steps described above.

Let's called this feature `instant ibc swap`.

## Concept

This contract is esenstially an ica module with its account being kinda like the ica account. However, the functionality is much more flexible and versatile than ica :

1. This contract account is not only controlled by only one remote account. It can be controlled by any remote account (weather that it's a contract account or just a regular account). But there's a catch, a remote account can only controll just the fund it sends to this contract account. For example, If remote contract A from juno chain send 50 juno to this contract account via ibc transfer, A can only use a total of 50 juno to `instant ibc swap`.


## Type of IBC Packet
SpotPriceQueryPacket

```
pub struct SpotPriceQueryPacket {
    pub pool_id: u64,
    pub in_denom: String,
    pub out_denom: String,
    pub with_swap_fee: bool,
}
```

IbcSwapPacket

```
pub struct IbcSwapPacket {
    pub pool_id: u64,
    pub in_amount: String,
    pub in_denom: String, 
    pub min_out_amount: String,
    pub out_denom: String,
    pub to_address: String,
}
```

## Type of execute message

SetIbcDenomForContractMsg

```
pub struct SetIbcDenomForContractMsg {
    pub ibc_denom: String,    
    pub contract_channel_id: String,
    pub contract_native_denom: String,
}
```
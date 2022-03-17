# How this contract works

## Motivation
If you want to swap some juno for some osmosis, you'll have to first send 1 ibc transfer transaction from juno to osmosis. Once the ibc transfer transaction is successful, your osmosis account receives the ibc juno token and you can execute swap transaction on osmosis chain.

This contract is deployed on osmosis. It enable swaping with using only 1 ibc transaction and you'll receive osmosis on your osmosis account as if you just do the 2 steps described above.

Let's called this feature `instant ibc swap`.

## Concept

If a remote `ibc contract` wants `instant ibc swap` feature, it will have to send its ibc coin to `this contract account`. Then, it will have to establish a channel connect to `this contract`. Let's called this channel `ibc-gamm channel`

This contract ensures that a `remote ibc contract` can controll a certain `ibc coin` held by `this contract account` only if that `ibc coin` comes from that `remote ibc contract`'s `transfer channel`.

How can we validate if an `ibc coin` belongs to a `remote ibc contract`'s `transfer channel`:

1. [`SetIbcDenomForContractMsg`](https://github.com/notional-labs/gamm-bounty/blob/af61c1d62afcf045a167360c093320a79080696c/contracts/ibc-gamm-osmosis/src/msg.rs#L22) once executed will tie an ibc denom to its respective `transfer channel` 's source chain `port id` and dest chain `connection id` (source chain is the coin native chain, dest chain is osmosis). This key value is saved.

2. If a `remote ibc contract` wants to swap on a osmosis pool using a certain `ibc coin` held by `this contract account`, it will have to send an [`IbcSwapPacket`](https://github.com/notional-labs/gamm-bounty/blob/af61c1d62afcf045a167360c093320a79080696c/contracts/ibc-gamm-osmosis/src/msg.rs#L42) on its `ibc-gamm channel`. Upon receiving [`IbcSwapPacket`](https://github.com/notional-labs/gamm-bounty/blob/af61c1d62afcf045a167360c093320a79080696c/contracts/ibc-gamm-osmosis/src/msg.rs#L42), this contract check if the `port id` of that `remote ibc contract` and the `connection id` on osmosis of its `ibc-gamm channel` match the source chain `port id` and dest chain `connection id` tied to that `ibc coin`'s denom (source chain `port id` and dest chain `connection id` for an ibc denom is set like what I mentioned above). If true, this contract will execute swap.

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

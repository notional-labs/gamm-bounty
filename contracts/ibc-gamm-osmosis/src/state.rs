use cw_storage_plus::{Map};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use cosmwasm_std::{ Storage};


pub const IBC_DENOM_TO_PORT_ID: Map<&str, String> = Map::new("IBC_DENOM_TO_PORT_ID");

pub const SWAP_QUEUE: &[&str;20] = &["";20];
pub const PREFIX_SWAP_QUEUE: &[u8] = b"swap_queue";

pub const KEY_SWAP_QUEUE_COUNTER: &[u8] = b"swap_queue_counter";



pub fn swap_queue(storage: &mut dyn Storage) -> Bucket<String> {
    bucket(storage, PREFIX_SWAP_QUEUE)
}

pub fn aswap_queue_read(storage: &dyn Storage) -> ReadonlyBucket<String> {
    bucket_read(storage, PREFIX_SWAP_QUEUE)
}

pub fn swap_queue_counter(storage: &mut dyn Storage) -> Singleton<u8> {
    singleton(storage, KEY_SWAP_QUEUE_COUNTER)
}

pub fn swap_queue_counter_read(storage: &dyn Storage) -> ReadonlySingleton<u8> {
    singleton_read(storage, KEY_SWAP_QUEUE_COUNTER)
}

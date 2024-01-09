use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const SIGNATORIES: Item<Vec<Addr>> = Item::new("signatories");
pub const THRESHOLD: Item<Uint128> = Item::new("threshold");
pub const TXS: Item<Vec<TX>> = Item::new("TXS");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TX {
    pub id: Uint128,
    pub to: Addr,
    pub value: Coin,
    pub approval_count: Uint128,
    pub approvals: Vec<Addr>,
    pub completed: bool,
}

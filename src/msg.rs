use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Uint128};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstantiateMsg {
    pub signatories: Vec<Addr>,
    pub threshold: Uint128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QueryMsg {
    GetThresold {},
    GetSignatories {},
    GetTx { id: Uint128 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExecuteMsg {
    CreateTx { tx: CreateTxInfo },
    ApproveTx { id: usize },
    SendFund { id: usize },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTxInfo {
    pub to: Addr,
    pub value: Coin,
}

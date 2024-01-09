use std::ops::Add;

use cosmwasm_std::{BankMsg, DepsMut, MessageInfo, Response, StdError, StdResult, Uint128};

use crate::{
    msg::CreateTxInfo,
    state::{THRESHOLD, TX, TXS},
    utils::{check_owner, check_signatory},
};

pub fn create_tx(deps: DepsMut, info: MessageInfo, tx: CreateTxInfo) -> StdResult<Response> {
    if !check_owner(info, deps.as_ref()) {
        return Err(StdError::GenericErr {
            msg: "sender not owner".to_string(),
        });
    }

    let mut txs = TXS.load(deps.storage)?;
    let tx = TX {
        id: Uint128::new(txs.len().to_string().parse::<u128>().unwrap()),
        to: tx.to,
        value: tx.value,
        approval_count: Uint128::new(0),
        approvals: Vec::new(),
        completed: false,
    };

    txs.push(tx);
    TXS.save(deps.storage, &txs)?;

    Ok(Response::new())
}

pub fn approve_tx(deps: DepsMut, info: MessageInfo, id: usize) -> StdResult<Response> {
    if !check_signatory(info.clone(), deps.as_ref()) {
        return Err(StdError::GenericErr {
            msg: "sender not signatory".to_string(),
        });
    }

    let mut txs = TXS.load(deps.storage)?;
    let tx = txs.get_mut(id).unwrap();
    if tx.approvals.contains(&info.sender) {
        return Err(StdError::GenericErr {
            msg: "sender already approved".to_string(),
        });
    }
    tx.approval_count = tx.approval_count.add(Uint128::new(1));
    tx.approvals.push(info.sender);
    TXS.save(deps.storage, &txs)?;

    Ok(Response::new())
}

pub fn send_fund(deps: DepsMut, info: MessageInfo, id: usize) -> StdResult<Response> {
    if !check_owner(info.clone(), deps.as_ref()) {
        return Err(StdError::GenericErr {
            msg: "sender not owner".to_string(),
        });
    }

    let mut txs = TXS.load(deps.storage)?;
    let threshold = THRESHOLD.load(deps.storage)?;
    let tx = txs.get_mut(id).unwrap();
    if tx.approval_count != threshold {
        return Err(StdError::GenericErr {
            msg: "threshold not reached".to_string(),
        });
    }

    if tx.completed {
        return Err(StdError::GenericErr {
            msg: "tx completed".to_string(),
        });
    }

    let send_msg = BankMsg::Send {
        to_address: tx.to.to_string(),
        amount: vec![tx.value.clone()],
    };

    tx.completed = true;
    TXS.save(deps.storage, &txs)?;

    Ok(Response::new().add_message(send_msg))
}

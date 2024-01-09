use cosmwasm_std::{to_json_binary, Binary, Deps, StdResult};

use crate::state::{SIGNATORIES, THRESHOLD, TXS};

pub fn query_threshold(deps: Deps) -> StdResult<Binary> {
    let thresold = THRESHOLD.load(deps.storage)?;
    to_json_binary(&thresold)
}

pub fn query_signatories(deps: Deps) -> StdResult<Binary> {
    let signatories = SIGNATORIES.load(deps.storage)?;
    to_json_binary(&signatories)
}

pub fn query_tx(deps: Deps, id: usize) -> StdResult<Binary> {
    let txs = TXS.load(deps.storage)?;
    let tx = txs.get(id);
    to_json_binary(&tx)
}

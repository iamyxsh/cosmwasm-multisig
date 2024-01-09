use cosmwasm_std::{Deps, MessageInfo};

use crate::state::{OWNER, SIGNATORIES};

pub fn check_owner(info: MessageInfo, deps: Deps) -> bool {
    let owner = OWNER.load(deps.storage).unwrap();
    if info.sender != owner {
        return false;
    } else {
        return true;
    }
}

pub fn check_signatory(info: MessageInfo, deps: Deps) -> bool {
    let signatories = SIGNATORIES.load(deps.storage).unwrap();
    if signatories.contains(&info.sender) {
        return true;
    } else {
        return false;
    }
}

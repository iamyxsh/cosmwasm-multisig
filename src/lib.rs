mod execute;
mod msg;
mod query;
mod resp;
mod state;
mod utils;

use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use query::{query_signatories, query_threshold, query_tx};
use state::{OWNER, SIGNATORIES, THRESHOLD, TX, TXS};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let txs: Vec<TX> = Vec::new();
    THRESHOLD.save(deps.storage, &msg.threshold)?;
    OWNER.save(deps.storage, &info.sender)?;
    SIGNATORIES.save(deps.storage, &msg.signatories)?;
    TXS.save(deps.storage, &txs)?;

    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSignatories {} => query_signatories(deps),
        QueryMsg::GetThresold {} => query_threshold(deps),
        QueryMsg::GetTx { id } => query_tx(deps, id.to_string().parse::<usize>().unwrap()),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateTx { tx } => execute::create_tx(deps, info, tx),
        ExecuteMsg::ApproveTx { id } => execute::approve_tx(deps, info, id),
        ExecuteMsg::SendFund { id } => execute::send_fund(deps, info, id),
    }
}

#[cfg(test)]
mod tests {

    use crate::{msg::CreateTxInfo, state::TX};

    use super::*;
    use cosmwasm_std::{coin, coins, Addr, Uint128};
    use cw_multi_test::{App, ContractWrapper, Executor};

    const SENDING_AMOUNT: u128 = 1;
    const THRESHOLD: Uint128 = Uint128::new(2);
    const SENDING_AMOUNT_DENOM: &str = "ATOM";
    const CONTRACT_NAME: &str = "Multisig Contract";

    const OWNER: &str = "owner";
    const SIGNER1: &str = "signer-1";
    const SIGNER2: &str = "signer-2";

    fn return_app() -> (App, Addr) {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(OWNER),
                    coins(SENDING_AMOUNT + 1, SENDING_AMOUNT_DENOM),
                )
                .unwrap()
        });
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(OWNER),
                &InstantiateMsg {
                    signatories: return_signatories(),
                    threshold: THRESHOLD,
                },
                &[],
                CONTRACT_NAME,
                None,
            )
            .unwrap();
        (app, addr)
    }

    fn return_signatories() -> Vec<Addr> {
        return vec![
            Addr::unchecked(OWNER),
            Addr::unchecked(SIGNER2),
            Addr::unchecked(SIGNER1),
        ];
    }

    #[test]
    fn it_can_instantiate() {
        let (app, addr) = return_app();

        let resp: Uint128 = app
            .wrap()
            .query_wasm_smart(addr.clone(), &QueryMsg::GetThresold {})
            .unwrap();

        assert_eq!(resp, Uint128::new(2));

        let resp: Vec<Addr> = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::GetSignatories {})
            .unwrap();

        assert_eq!(resp, return_signatories());
    }

    #[test]
    fn it_can_create_tx() {
        let (mut app, addr) = return_app();

        let tx = CreateTxInfo {
            to: Addr::unchecked(SIGNER1),
            value: coin(SENDING_AMOUNT, SENDING_AMOUNT_DENOM),
        };

        app.execute_contract(
            Addr::unchecked(OWNER),
            addr.clone(),
            &ExecuteMsg::CreateTx { tx },
            &[coin(SENDING_AMOUNT, SENDING_AMOUNT_DENOM)],
        )
        .unwrap();

        let resp: TX = app
            .wrap()
            .query_wasm_smart(
                addr,
                &QueryMsg::GetTx {
                    id: Uint128::new(0),
                },
            )
            .unwrap();

        assert_eq!(resp.value, coin(SENDING_AMOUNT, SENDING_AMOUNT_DENOM));
        assert_eq!(resp.to, SIGNER1);
        assert_eq!(resp.approval_count, Uint128::new(0));
        assert_eq!(resp.approvals.len(), 0);
    }

    #[test]
    fn it_can_approve_tx() {
        let (mut app, addr) = return_app();

        let tx = CreateTxInfo {
            to: Addr::unchecked(SIGNER1),
            value: coin(SENDING_AMOUNT, SENDING_AMOUNT_DENOM),
        };

        app.execute_contract(
            Addr::unchecked(OWNER),
            addr.clone(),
            &ExecuteMsg::CreateTx { tx },
            &[coin(SENDING_AMOUNT, SENDING_AMOUNT_DENOM)],
        )
        .unwrap();

        app.execute_contract(
            Addr::unchecked(OWNER),
            addr.clone(),
            &ExecuteMsg::ApproveTx { id: 0 },
            &[],
        )
        .unwrap();

        app.execute_contract(
            Addr::unchecked(SIGNER2),
            addr.clone(),
            &ExecuteMsg::ApproveTx { id: 0 },
            &[],
        )
        .unwrap();

        let resp: TX = app
            .wrap()
            .query_wasm_smart(
                addr,
                &QueryMsg::GetTx {
                    id: Uint128::new(0),
                },
            )
            .unwrap();

        assert_eq!(resp.approval_count, Uint128::new(2));
        assert_eq!(resp.approvals.len(), 2);
        assert_eq!(
            resp.approvals,
            vec![Addr::unchecked(OWNER), Addr::unchecked(SIGNER2)]
        );
    }

    #[test]
    fn it_can_send_funds() {
        let (mut app, addr) = return_app();

        let tx = CreateTxInfo {
            to: Addr::unchecked(SIGNER1),
            value: coin(SENDING_AMOUNT, SENDING_AMOUNT_DENOM),
        };

        app.execute_contract(
            Addr::unchecked(OWNER),
            addr.clone(),
            &ExecuteMsg::CreateTx { tx },
            &[coin(SENDING_AMOUNT, SENDING_AMOUNT_DENOM)],
        )
        .unwrap();

        app.execute_contract(
            Addr::unchecked(OWNER),
            addr.clone(),
            &ExecuteMsg::ApproveTx { id: 0 },
            &[],
        )
        .unwrap();

        app.execute_contract(
            Addr::unchecked(SIGNER2),
            addr.clone(),
            &ExecuteMsg::ApproveTx { id: 0 },
            &[],
        )
        .unwrap();

        app.execute_contract(
            Addr::unchecked(OWNER),
            addr.clone(),
            &ExecuteMsg::SendFund { id: 0 },
            &[],
        )
        .unwrap();

        let resp: TX = app
            .wrap()
            .query_wasm_smart(
                addr,
                &QueryMsg::GetTx {
                    id: Uint128::new(0),
                },
            )
            .unwrap();

        assert!(resp.completed);

        let balance = app
            .wrap()
            .query_balance(resp.to, SENDING_AMOUNT_DENOM)
            .unwrap();

        assert_eq!(balance.amount, Uint128::new(1));
    }
}

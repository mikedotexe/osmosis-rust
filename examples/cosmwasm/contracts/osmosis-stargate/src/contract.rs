use std::convert::TryInto;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    CosmosMsg, DepsMut, Env, MessageInfo, Reply, Response, SubMsg, SubMsgResponse, SubMsgResult,
};
use cw2::set_contract_version;
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::osmosis::gamm::poolmodels::balancer::v1beta1::{
    MsgCreateBalancerPool, MsgCreateBalancerPoolResponse,
};
use osmosis_std::types::osmosis::gamm::v1beta1::{PoolAsset, PoolParams};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgMint, QueryDenomsFromCreatorRequest, QueryDenomsFromCreatorResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InitPoolCfg, InstantiateMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:osmosis-stargate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const CREATE_POOL_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateDenom {
            subdenom,
            initial_mint,
            initial_pool,
        } => try_create_denom(env, subdenom, initial_mint, initial_pool),
    }
}

pub fn try_create_denom(
    env: Env,
    subdenom: String,
    initial_mint: Option<String>,
    initial_pool: Option<InitPoolCfg>,
) -> Result<Response, ContractError> {
    let contract_addr = env.contract.address.to_string();

    let msg_create_denom: CosmosMsg = MsgCreateDenom {
        sender: contract_addr.clone(),
        subdenom: subdenom.clone(),
    }
    .into();

    let mut msgs = vec![SubMsg::new(msg_create_denom)];

    if let Some(initial_mint) = initial_mint {
        let msg_mint: CosmosMsg = MsgMint {
            sender: contract_addr.clone(),
            amount: Some(Coin {
                denom: format!("factory/{contract_addr}/{subdenom}"),
                amount: initial_mint,
            }),
        }
        .into();

        msgs.push(SubMsg::new(msg_mint));

        if let Some(InitPoolCfg {
            swap_fee,
            exit_fee,
            pairing_denom,
            pool_assets,
        }) = initial_pool
        {
            let msg_create_pool: CosmosMsg = MsgCreateBalancerPool {
                sender: contract_addr.clone(),
                pool_params: PoolParams {
                    swap_fee,
                    exit_fee,
                    smooth_weight_change_params: None,
                }
                .into(),
                pool_assets: vec![
                    PoolAsset {
                        token: Coin {
                            denom: format!("factory/{contract_addr}/{subdenom}"),
                            amount: pool_assets.new_token_amount,
                        }
                        .into(),
                        weight: pool_assets.new_token_weight,
                    },
                    PoolAsset {
                        token: Coin {
                            denom: pairing_denom,
                            amount: pool_assets.pairing_token_amount,
                        }
                        .into(),
                        weight: pool_assets.pairing_token_weight,
                    },
                ],
                future_pool_governor: contract_addr,
            }
            .into();

            msgs.push(SubMsg::reply_on_success(
                msg_create_pool,
                CREATE_POOL_REPLY_ID,
            ));
        }
    };

    Ok(Response::new()
        .add_submessages(msgs)
        .add_attribute("method", "try_create_denom"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id == CREATE_POOL_REPLY_ID {
        if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
            // This is only for response deserialization demonstration purpose.
            // `pool_id` can actually be retreived from `pool_created` event.
            let res: MsgCreateBalancerPoolResponse = b.try_into().map_err(ContractError::Std)?;
            return Ok(Response::new().add_attribute("pool_id", format!("{}", res.pool_id)));
        }
    };

    Ok(Response::new())
}

pub fn try_create_balancer_pool(env: Env, subdenom: String) -> Result<Response, ContractError> {
    let contract_addr = env.contract.address.to_string();
    let pool_params = PoolParams {
        swap_fee: "1".into(),
        exit_fee: "1".into(),
        smooth_weight_change_params: None,
    }
    .into();
    let msg: CosmosMsg = MsgCreateBalancerPool {
        sender: contract_addr.clone(),
        pool_params,
        pool_assets: vec![
            PoolAsset {
                token: Coin {
                    denom: "uosmo".into(),
                    amount: "100000000".into(),
                }
                .into(),
                weight: "1".into(),
            },
            PoolAsset {
                token: Coin {
                    denom: format!("factory/{contract_addr}/{subdenom}"),
                    amount: "100000000".into(),
                }
                .into(),
                weight: "1".into(),
            },
        ],
        future_pool_governor: contract_addr,
    }
    .into();

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "try_create_denom"))
}

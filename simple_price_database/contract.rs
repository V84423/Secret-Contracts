pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    owner(&mut deps.storage).save(&deps.api.canonical_address(&env.message.sender)?)?;
    oracle_ref(&mut deps.storage).save(&deps.api.canonical_address(&msg.initial_oracle_ref)?)?;
    Ok(InitResponse::default())
}


fn query_owner<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CanonicalAddr> {
    owner_read(&deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("OWNER_NOT_INITIALIZED"))
}

fn query_oracle_ref<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CanonicalAddr> {
    oracle_ref_read(&deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("ORACLE_REF_NOT_INITIALIZED"))
}

fn query_price<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    symbol: String,
)  -> StdResult<Uint128> {
    match price_read(&deps.storage).get(&symbol.as_bytes()) {
        Some(data) => {
            Ok(bincode::deserialize(&data).unwrap())
        },
        _ => Err(StdError::generic_err(format!(
            "PRICE_NOT_AVAILABLE_FOR_KEY:{}",
            symbol
        ))),
    }
}


// cross-contract query
fn query_reference_data<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    base_symbol: String,
    quote_symbol: String,
) -> StdResult<ReferenceData> {
    Ok(deps.querier.custom_query::<QueryMsg, ReferenceData>(
        &WasmQuery::Smart {
            contract_addr: deps.api.human_address(&query_oracle_ref(deps)?)?,
            msg: to_binary(&QueryExtMsg::GetReferenceData {
                base_symbol,
                quote_symbol,
            })?,
        }
        .into(),
    )?)
}



pub fn try_set_oracle_ref<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    new_oracle_ref: HumanAddr,
) -> StdResult<HandleResponse> {
    let owner_addr = owner(&mut deps.storage).load()?;
    if deps.api.canonical_address(&env.message.sender)? != owner_addr {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    oracle_ref(&mut deps.storage).save(&deps.api.canonical_address(&new_oracle_ref)?)?;

    Ok(HandleResponse::default())
}

pub fn try_set_price<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    symbol: String,
) -> StdResult<HandleResponse> {
    let owner_addr = owner(&mut deps.storage).load()?;
    if deps.api.canonical_address(&env.message.sender)? != owner_addr {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    let reference_data = query_reference_data(deps, symbol.clone(), "USD".into())?;
    price(&mut deps.storage).set(symbol.as_bytes(), &bincode::serialize(&reference_data.rate).unwrap());

    Ok(HandleResponse::default())
}
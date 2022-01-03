use cosmwasm_std::{
    to_binary, Api, Env, Extern, HandleResult, InitResponse, Querier, QueryResult, StdResult,
    Storage,
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Message: DeserializeOwned + JsonSchema {}

pub trait Response: Serialize + Clone + std::fmt::Debug + PartialEq + JsonSchema {}

pub trait CustomMsg<'de>:
    Serialize + Deserialize<'de> + Clone + std::fmt::Debug + PartialEq + JsonSchema
{
}

pub trait Contract {
    type InitMsg: Message;

    type HandleMsg: Message;
    type HandleResponse: Response;

    type QueryMsg: Message;
    type QueryResponse: Response;

    fn init<S, A, Q>(
        deps: &mut Extern<S, A, Q>,
        env: Env,
        msg: Self::InitMsg,
    ) -> StdResult<InitResponse>
    where
        S: Storage,
        A: Api,
        Q: Querier;

    fn handle<S, A, Q>(deps: &mut Extern<S, A, Q>, env: Env, msg: Self::HandleMsg) -> HandleResult
    where
        S: Storage,
        A: Api,
        Q: Querier;

    fn query<S, A, Q>(
        deps: &Extern<S, A, Q>,
        msg: Self::QueryMsg,
    ) -> StdResult<Self::QueryResponse>
    where
        S: Storage,
        A: Api,
        Q: Querier;

    fn _query<S, A, Q>(deps: &Extern<S, A, Q>, msg: Self::QueryMsg) -> QueryResult
    where
        S: Storage,
        A: Api,
        Q: Querier,
    {
        Self::query(deps, msg).and_then(|response| to_binary(&response))
    }
}

impl<T> Message for T where T: DeserializeOwned + JsonSchema {}

impl<T> Response for T where T: Serialize + Clone + std::fmt::Debug + PartialEq + JsonSchema {}

impl<'de, T> CustomMsg<'de> for T where
    T: Serialize + Deserialize<'de> + Clone + std::fmt::Debug + PartialEq + JsonSchema
{
}

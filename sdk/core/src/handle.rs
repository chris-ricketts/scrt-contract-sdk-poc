use cosmwasm_std::{to_binary, CosmosMsg, HandleResponse, HandleResult, LogAttribute};

use crate::contract::{CustomMsg, Response};

pub trait HandleResultExt<'de, R>
where
    R: Response,
{
    type CustomMsg: CustomMsg<'de>;

    fn from_response(response: R) -> HandleResult<Self::CustomMsg> {
        let mut res = HandleResponse::default();
        let data = to_binary(&response)?;
        res.data = Some(data);
        Ok(res)
    }

    fn push_msg(self, msg: CosmosMsg<Self::CustomMsg>) -> HandleResult<Self::CustomMsg>;

    fn push_log(self, log: LogAttribute) -> HandleResult<Self::CustomMsg>;
}

impl<'de, R, T> HandleResultExt<'de, R> for HandleResult<T>
where
    R: Response,
    T: CustomMsg<'de>,
{
    type CustomMsg = T;

    fn push_msg(self, msg: CosmosMsg<Self::CustomMsg>) -> HandleResult<Self::CustomMsg> {
        let mut res = self?;
        res.messages.push(msg);
        Ok(res)
    }

    fn push_log(self, log: LogAttribute) -> HandleResult<Self::CustomMsg> {
        let mut res = self?;
        res.log.push(log);
        Ok(res)
    }
}

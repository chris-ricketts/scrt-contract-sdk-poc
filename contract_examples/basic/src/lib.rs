use scrt_contract_sdk::prelude::*;

mod msg;
mod state;

struct Basic;

fn handle_record<S, A, Q>(deps: &mut Extern<S, A, Q>, env: Env, reminder: String) -> HandleResult
where
    S: Storage,
    A: Api,
    Q: Querier,
{
    let config = state::Config::load(&deps.storage)?;

    if reminder.as_bytes().len() > config.max_size {
        let response = msg::HandleResponse::record_status(format!(
            "Reminder byte length exceeds maximum of {} bytes",
            config.max_size
        ));
        return HandleResult::from_response(response);
    }

    let reminder = state::Reminder {
        content: reminder.into_bytes(),
        timestamp: env.block.time,
    };

    let sender = deps.api.canonical_address(&env.message.sender)?;

    reminder
        .save_for_address(sender)
        .execute_with_storage(&mut deps.storage)?;

    state::State::update(|state| state.reminder_count += 1, &mut deps.storage)?;

    let response = msg::HandleResponse::record_status("Reminder recorded!");
    HandleResult::from_response(response)
}

fn handle_read<S, A, Q>(deps: &mut Extern<S, A, Q>, env: Env) -> HandleResult
where
    S: Storage,
    A: Api,
    Q: Querier,
{
    let sender = deps.api.canonical_address(&env.message.sender)?;
    match state::Reminder::load_for_address(sender).execute_with_storage(&deps.storage) {
        Ok(reminder) => {
            // This is fine to unwrap as we know any stored reminder came from a valid UTF-8 string
            let reminder_str = String::from_utf8(reminder.content).unwrap();
            let response = msg::HandleResponse::read_status_found(reminder_str, reminder.timestamp);
            HandleResult::from_response(response)
        }
        Err(err) if err.is_not_found() => {
            let response = msg::HandleResponse::read_status_not_found();
            HandleResult::from_response(response)
        }
        Err(err) => Err(err),
    }
}

impl Contract for Basic {
    type InitMsg = msg::InitMsg;

    type HandleMsg = msg::HandleMsg;
    type HandleResponse = msg::HandleResponse;

    type QueryMsg = msg::QueryMsg;
    type QueryResponse = msg::QueryResponse;

    fn init<S, A, Q>(
        deps: &mut Extern<S, A, Q>,
        _env: Env,
        msg: Self::InitMsg,
    ) -> StdResult<InitResponse>
    where
        S: Storage,
        A: Api,
        Q: Querier,
    {
        let config = state::Config {
            max_size: msg.max_size,
        };
        config.save(&mut deps.storage)?;
        Ok(InitResponse::default())
    }

    fn handle<S, A, Q>(deps: &mut Extern<S, A, Q>, env: Env, msg: Self::HandleMsg) -> HandleResult
    where
        S: Storage,
        A: Api,
        Q: Querier,
    {
        match msg {
            msg::HandleMsg::Record { reminder } => handle_record(deps, env, reminder),
            msg::HandleMsg::Read {} => handle_read(deps, env),
        }
    }

    fn query<S, A, Q>(deps: &Extern<S, A, Q>, msg: Self::QueryMsg) -> StdResult<Self::QueryResponse>
    where
        S: Storage,
        A: Api,
        Q: Querier,
    {
        match msg {
            msg::QueryMsg::Stats {} => {
                let state = state::State::load(&deps.storage)?;
                Ok(msg::QueryResponse::stats(state.reminder_count))
            }
        }
    }
}

hookup_contract!(Basic);

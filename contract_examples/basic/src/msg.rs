use scrt_contract_sdk::prelude::*;

// TODO: Can we simplify this derive and still use the serde imported from the sdk crate?
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(crate = "self::serde")] // Required because serde is rexported from sdk crate
pub struct InitMsg {
    pub max_size: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", crate = "self::serde")]
pub enum HandleMsg {
    /// Records a new reminder for the sender
    Record { reminder: String },
    /// Requests the current reminder for the sender
    Read {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", crate = "self::serde")]
pub enum QueryMsg {
    /// Gets basic statistics about the use of the contract
    Stats {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
#[serde(rename_all = "snake_case", crate = "self::serde")]
pub enum HandleResponse {
    /// Return a status message to let the user know if it succeeded or failed
    Record { status: String },
    /// Return a status message and the current reminder and its timestamp, if it exists
    Read {
        status: String,
        reminder: Option<String>,
        timestamp: Option<u64>,
    },
}

impl HandleResponse {
    pub(crate) fn record_status<S: Into<String>>(status: S) -> HandleResponse {
        HandleResponse::Record {
            status: status.into(),
        }
    }

    pub(crate) fn read_status_not_found() -> HandleResponse {
        HandleResponse::Read {
            status: String::from("No reminders found."),
            reminder: None,
            timestamp: None,
        }
    }

    pub(crate) fn read_status_found(reminder: String, timestamp: u64) -> HandleResponse {
        HandleResponse::Read {
            status: String::from("Reminder found!"),
            reminder: Some(reminder),
            timestamp: Some(timestamp),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
#[serde(rename_all = "snake_case", crate = "self::serde")]
pub enum QueryResponse {
    /// Return basic statistics about contract
    Stats { reminder_count: u64 },
}

impl QueryResponse {
    pub(crate) fn stats(reminder_count: u64) -> QueryResponse {
        QueryResponse::Stats { reminder_count }
    }
}

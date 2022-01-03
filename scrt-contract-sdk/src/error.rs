use cosmwasm_std::StdError;

pub trait StdErrorExt {
    fn is_not_found(&self) -> bool;

    fn is_invalid_base64(&self) -> bool;

    fn is_invalid_utf8(&self) -> bool;

    fn is_parse_err(&self) -> bool;

    fn is_serialize_err(&self) -> bool;

    fn is_unauthorized(&self) -> bool;

    fn is_underflow(&self) -> bool;
}

impl StdErrorExt for StdError {
    fn is_not_found(&self) -> bool {
        matches!(self, StdError::NotFound { .. })
    }

    fn is_invalid_base64(&self) -> bool {
        matches!(self, StdError::InvalidBase64 { .. })
    }

    fn is_invalid_utf8(&self) -> bool {
        matches!(self, StdError::InvalidUtf8 { .. })
    }

    fn is_parse_err(&self) -> bool {
        matches!(self, StdError::ParseErr { .. })
    }

    fn is_serialize_err(&self) -> bool {
        matches!(self, StdError::SerializeErr { .. })
    }

    fn is_unauthorized(&self) -> bool {
        matches!(self, StdError::Unauthorized { .. })
    }

    fn is_underflow(&self) -> bool {
        matches!(self, StdError::Underflow { .. })
    }
}

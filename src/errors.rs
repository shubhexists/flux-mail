pub struct SmtpResponseError<'a> {
    code: &'a SmtpErrorCode,
    message: &'a str,
}

impl<'a> SmtpResponseError<'a> {
    pub fn new(code: &'a SmtpErrorCode) -> Self {
        Self {
            code,
            message: code.as_message(),
        }
    }
}

pub enum SmtpErrorCode {
    SyntaxError,
    CommandUnrecognized,
    InvalidParameters,
    MailboxUnavailable,
    InsufficientSystemStorage,
    ActionNotTaken,
    MessageSizeExceedsLimit,
    TransactionFailed,
}

impl SmtpErrorCode {
    fn as_code(&self) -> u16 {
        match self {
            SmtpErrorCode::SyntaxError => 500,
            SmtpErrorCode::CommandUnrecognized => 500,
            SmtpErrorCode::InvalidParameters => 501,
            SmtpErrorCode::MailboxUnavailable => 550,
            SmtpErrorCode::InsufficientSystemStorage => 452,
            SmtpErrorCode::ActionNotTaken => 550,
            SmtpErrorCode::MessageSizeExceedsLimit => 552,
            SmtpErrorCode::TransactionFailed => 554,
        }
    }

    fn as_message(&self) -> &str {
        match self {
            SmtpErrorCode::SyntaxError => "Syntax error, command unrecognized",
            SmtpErrorCode::CommandUnrecognized => "Command unrecognized",
            SmtpErrorCode::InvalidParameters => "Syntax error in parameters or arguments",

            SmtpErrorCode::MailboxUnavailable => "Requested action not taken (mailbox unavailable)",

            SmtpErrorCode::InsufficientSystemStorage => {
                "Requested action not taken (insufficient system storage)"
            }
            SmtpErrorCode::ActionNotTaken => "Requested action not taken",

            SmtpErrorCode::MessageSizeExceedsLimit => {
                "Requested action aborted (message size exceeds limit)"
            }
            SmtpErrorCode::TransactionFailed => "Transaction failed",
        }
    }
}

impl Into<u16> for SmtpErrorCode {
    fn into(self) -> u16 {
        self.as_code()
    }
}

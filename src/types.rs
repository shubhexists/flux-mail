use crate::errors::SmtpResponseError;

pub struct Email<'a> {
    pub sender: &'a str,
    pub recipients: Vec<&'a str>,
    pub content: &'a str,
    pub size: usize,
}

pub enum CurrentStates<'a> {
    Initial,
    Greeted,
    AwaitingRecipient(Email<'a>),
    AwaitingData(Email<'a>),
    DataReceived(Email<'a>),
}

pub type SMTPResult<'a, T> = Result<&'a T, SmtpResponseError<'a>>;

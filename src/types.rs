use crate::errors::SmtpResponseError;

#[derive(Default, Clone)]
pub struct Email {
    #[allow(dead_code)]
    pub sender: String,
    pub recipients: Vec<String>,
    pub content: String,
    pub size: usize,
}

pub enum CurrentStates {
    Initial,
    Greeted,
    AwaitingRecipient(Email),
    AwaitingData(Email),
    #[allow(dead_code)]
    DataReceived(Email),
}

pub type SMTPResult<'a, T> = Result<T, SmtpResponseError<'a>>;

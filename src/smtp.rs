use crate::{
    errors::{SmtpErrorCode, SmtpResponseError},
    is_valid_email,
    types::{CurrentStates, Email, SMTPResult},
    CLOSING_CONNECTION, DATA_READY_PROMPT, MAX_EMAIL_SIZE, MAX_RECIPIENT_COUNT, SUCCESS_RESPONSE,
};
use std::str::SplitWhitespace;

struct HandleCurrentState<'a> {
    current_state: CurrentStates<'a>,
    greeting_message: String,
    max_email_size: usize,
}

impl<'a> HandleCurrentState<'a> {
    pub fn new(server_domain: impl AsRef<str>) -> Self {
        let server_domain: &str = server_domain.as_ref();
        let greeting_message: String = format!(
            "250-{server_domain} greets {server_domain}\n\
             250-SIZE {}\n\
             250 8BITMIME\n",
            MAX_EMAIL_SIZE
        );

        Self {
            current_state: CurrentStates::Initial,
            max_email_size: MAX_EMAIL_SIZE,
            greeting_message,
        }
    }

    pub fn process_smtp_command(&'a mut self, client_message: &'a str) -> SMTPResult<'a, [u8]> {
        let message: &str = client_message.trim();
        if message.is_empty() {
            return Err(SmtpResponseError::new(&SmtpErrorCode::SyntaxError));
        }
        let mut message_parts: SplitWhitespace<'_> = message.split_whitespace();
        let command: String = message_parts
            .next()
            .ok_or_else(|| SmtpResponseError::new(&SmtpErrorCode::SyntaxError))?
            .to_lowercase();

        let previous_state: CurrentStates<'a> =
            std::mem::replace(&mut self.current_state, CurrentStates::Initial);
        match (command.as_str(), previous_state) {
            ("helo" | "ehlo", CurrentStates::Initial) => {
                self.current_state = CurrentStates::Greeted;
                Ok(self.greeting_message.as_bytes())
            }
            ("mail", CurrentStates::Greeted) => {
                let sender: &str = message_parts
                    .next()
                    .and_then(|s| s.strip_prefix("FROM:"))
                    .ok_or_else(|| SmtpResponseError::new(&SmtpErrorCode::InvalidParameters))?;

                match is_valid_email(sender) {
                    false => {
                        return Err(SmtpResponseError::new(&SmtpErrorCode::MailboxUnavailable))
                    }
                    true => {}
                }

                self.current_state = CurrentStates::AwaitingRecipient(Email {
                    sender,
                    ..Default::default()
                });
                Ok(SUCCESS_RESPONSE)
            }
            ("rcpt", CurrentStates::AwaitingRecipient(mut email)) => {
                if email.recipients.len() >= MAX_RECIPIENT_COUNT {
                    return Err(SmtpResponseError::new(
                        &SmtpErrorCode::InsufficientSystemStorage,
                    ));
                }
                let reciever: &str = message_parts
                    .next()
                    .and_then(|s| s.strip_prefix("TO:"))
                    .ok_or_else(|| SmtpResponseError::new(&SmtpErrorCode::InvalidParameters))?;

                match is_valid_email(reciever) {
                    false => {
                        return Err(SmtpResponseError::new(&SmtpErrorCode::MailboxUnavailable))
                    }
                    true => {}
                }

                email.recipients.push(reciever);
                self.current_state = CurrentStates::AwaitingRecipient(email);
                Ok(SUCCESS_RESPONSE)
            }
            ("data", CurrentStates::AwaitingRecipient(email)) => {
                match email.recipients.is_empty() {
                    true => return Err(SmtpResponseError::new(&SmtpErrorCode::TransactionFailed)),
                    false => {}
                };
                self.current_state = CurrentStates::AwaitingData(email);
                Ok(DATA_READY_PROMPT)
            }
            (_, CurrentStates::AwaitingData(mut email)) => {
                email.size += client_message.len();
                match email.size > MAX_EMAIL_SIZE {
                    true => {}
                    false => {
                        return Err(SmtpResponseError::new(
                            &SmtpErrorCode::MessageSizeExceedsLimit,
                        ));
                    }
                };

                let response: &[u8] = match client_message.ends_with("\r\n.\r\n") {
                    true => {
                        self.current_state =
                            CurrentStates::DataReceived(std::mem::take(&mut email));
                        SUCCESS_RESPONSE
                    }
                    false => {
                        self.current_state =
                            CurrentStates::AwaitingData(std::mem::take(&mut email));
                        b""
                    }
                };

                email.content += client_message;
                Ok(&response)
            }
            ("quit", _) => Ok(CLOSING_CONNECTION),
            _ => Err(SmtpResponseError::new(&SmtpErrorCode::CommandUnrecognized)),
        }
    }
}

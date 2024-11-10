use crate::{
    errors::{SmtpErrorCode, SmtpResponseError},
    is_valid_email,
    types::{CurrentStates, Email, SMTPResult},
    AUTH_OK, CLOSING_CONNECTION, DATA_READY_PROMPT, MAX_EMAIL_SIZE, MAX_RECIPIENT_COUNT,
    SUCCESS_RESPONSE,
};
use std::str::SplitWhitespace;

pub struct HandleCurrentState {
    current_state: CurrentStates,
    greeting_message: String,
    max_email_size: usize,
}

impl HandleCurrentState {
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

    pub fn process_smtp_command<'a>(&mut self, client_message: &str) -> SMTPResult<'a, &[u8]> {
        let message: &str = client_message.trim();
        if message.is_empty() {
            return Err(SmtpResponseError::new(&SmtpErrorCode::SyntaxError));
        }
        let mut message_parts: SplitWhitespace<'_> = message.split_whitespace();
        let command: String = message_parts
            .next()
            .ok_or_else(|| SmtpResponseError::new(&SmtpErrorCode::SyntaxError))?
            .to_lowercase();

        let previous_state: CurrentStates =
            std::mem::replace(&mut self.current_state, CurrentStates::Initial);
        match (command.as_str(), previous_state) {
            ("ehlo", CurrentStates::Initial) => {
                self.current_state = CurrentStates::Greeted;
                Ok(self.greeting_message.as_bytes())
            }
            ("helo", CurrentStates::Initial) => {
                self.current_state = CurrentStates::Greeted;
                Ok(SUCCESS_RESPONSE)
            }
            ("noop", _) | ("help", _) | ("info", _) | ("vrfy", _) | ("expn", _) => {
                Ok(SUCCESS_RESPONSE)
            }
            ("rset", _) => {
                self.current_state = CurrentStates::Initial;
                Ok(SUCCESS_RESPONSE)
            }
            ("auth", _) => Ok(AUTH_OK),
            ("mail", CurrentStates::Greeted) => {
                let sender: &str = message_parts
                    .next()
                    .and_then(|s: &str| s.strip_prefix("FROM:"))
                    .ok_or_else(|| SmtpResponseError::new(&SmtpErrorCode::InvalidParameters))?;

                if !is_valid_email(sender) {
                    return Err(SmtpResponseError::new(&SmtpErrorCode::MailboxUnavailable));
                }

                self.current_state = CurrentStates::AwaitingRecipient(Email {
                    sender: sender.to_string(),
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
                let receiver: &str = message_parts
                    .next()
                    .and_then(|s: &str| s.strip_prefix("TO:"))
                    .ok_or_else(|| SmtpResponseError::new(&SmtpErrorCode::InvalidParameters))?;

                if !is_valid_email(receiver) {
                    return Err(SmtpResponseError::new(&SmtpErrorCode::MailboxUnavailable));
                }

                email.recipients.push(receiver.to_string());
                self.current_state = CurrentStates::AwaitingRecipient(email);
                Ok(SUCCESS_RESPONSE)
            }
            ("data", CurrentStates::AwaitingRecipient(email)) => {
                if email.recipients.is_empty() {
                    return Err(SmtpResponseError::new(&SmtpErrorCode::TransactionFailed));
                }
                self.current_state = CurrentStates::AwaitingData(email);
                Ok(DATA_READY_PROMPT)
            }
            ("quit", CurrentStates::AwaitingData(mail)) => {
                self.current_state = CurrentStates::DataReceived(mail);
                Ok(CLOSING_CONNECTION)
            }
            ("quit", _) => Ok(CLOSING_CONNECTION),
            (_, CurrentStates::AwaitingData(mut email)) => {
                email.size += client_message.len();
                if email.size > self.max_email_size {
                    return Err(SmtpResponseError::new(
                        &SmtpErrorCode::MessageSizeExceedsLimit,
                    ));
                }

                let response: &[u8] = if client_message.ends_with("\r\n.\r\n") {
                    self.current_state = CurrentStates::DataReceived(std::mem::take(&mut email));
                    SUCCESS_RESPONSE
                } else {
                    self.current_state = CurrentStates::AwaitingData(std::mem::take(&mut email));
                    b""
                };

                email.content.push_str(client_message);
                Ok(response)
            }
            _ => Err(SmtpResponseError::new(&SmtpErrorCode::CommandUnrecognized)),
        }
    }
}

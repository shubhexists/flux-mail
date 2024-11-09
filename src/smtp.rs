use crate::{
    errors::{SmtpErrorCode, SmtpResponseError},
    types::{CurrentStates, SMTPResult},
    MAX_EMAIL_SIZE,
};

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

    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && !email.contains("..") && email.len() < 254
    }

    pub fn process_smtp_command(&mut self, client_message: &str) -> SMTPResult<'a, [u8]> {
        let message = client_message.trim();
        if message.is_empty() {
            return Err(SmtpResponseError::new(&SmtpErrorCode::SyntaxError));
        }
        todo!()
    }
}

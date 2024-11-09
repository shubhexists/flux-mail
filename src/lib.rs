mod errors;
mod smtp;
mod types;
use std::time::Duration;

const MAX_EMAIL_SIZE: usize = 10_485_760;
const TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RECIPIENT_COUNT: usize = 100;
const INITIAL_GREETING: &'static [u8] = b"220 ESMTP Service Ready\n";
const SUCCESS_RESPONSE: &'static [u8] = b"250 Ok\n";
const DATA_READY_PROMPT: &'static [u8] = b"354 End data with <CR><LF>.<CR><LF>\n";
const CLOSING_CONNECTION: &'static [u8] = b"221 Goodbye\n";

pub(crate) fn is_valid_email(email: &str) -> bool {
    email.contains('@') && !email.contains("..") && email.len() < 254
}

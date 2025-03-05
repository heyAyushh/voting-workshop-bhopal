use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid poll end time")]
    InvalidPollEndTime,
    #[msg("Poll end is not a valid unix timestamp")]
    InvalidUnixTimestamp
}
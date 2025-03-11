use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Poll inactive")]
    PollNotActive,
}
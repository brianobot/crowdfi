use anchor_lang::error_code;


#[error_code]
pub enum CrowdfiError {
    #[msg("Custom Error")]
    CustomError
}
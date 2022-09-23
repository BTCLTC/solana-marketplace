use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The contract frozen")]
    FreezeProgramError,

    #[msg("Fee Rate Error")]
    FeeRateError,

    #[msg("Invalid Request")]
    InvalidRequestError,

    #[msg("The sum of shares is not 100")]
    InvalidShares,

    #[msg("Metadata doesn't exist")]
    MetadataNotExist,

    #[msg("Invalid token amount")]
    InvalidTokenAmount,
}

use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The contract frozen")]
    FreezeProgramError,

    #[msg("Fee Rate Error")]
    FeeRateError,

    #[msg("Invalid Request")]
    InvalidRequestError,

    #[msg("Not enough SOL")]
    InsufficientSolAmountError,

    #[msg("The amount is small than min price")]
    InsufficientMinAmountError,

    #[msg("IncorrectOwner")]
    IncorrectOwner,

    #[msg("Derived key invalid")]
    DerivedKeyInvalid,

    #[msg("Metadata doesn't exist")]
    MetadataNotExist,

    #[msg("PublicKeyMismatch")]
    PublicKeyMismatch,

    #[msg("UninitializedAccount")]
    UninitializedAccount,

    #[msg("No payer present on this txn")]
    NoPayerPresent,

    #[msg("Invalid token amount")]
    InvalidTokenAmount,
}

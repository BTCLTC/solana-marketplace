use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Permission Error, E1000")]
    PermissionError,

    #[msg("The contract frozen, E1001")]
    FreezeProgramError,

    #[msg("The token frozen, E1002")]
    FreezeTokenError,

    #[msg("NFT Locked, E1003")]
    NFTLockedError,

    #[msg("Invalid Request, E1004")]
    InvalidRequestError,

    #[msg("Trade not available, E1005")]
    TradeNotAvailableError,

    #[msg("Not exist member, E1006")]
    NoMemberError,

    #[msg("Not enough SOL, E1007")]
    InsufficientSolAmountError,

    #[msg("Not enough Token, E1008")]
    InsufficientTokenAmountError,

    #[msg("The amount is small than min price, E1009")]
    InsufficientMinAmountError,

    #[msg("IncorrectOwner, E1010")]
    IncorrectOwner,

    #[msg("Derived key invalid, E1011")]
    DerivedKeyInvalid,

    #[msg("Metadata doesn't exist, E1012")]
    MetadataDoesntExist,

    #[msg("PublicKeyMismatch, E1013")]
    PublicKeyMismatch,

    #[msg("UninitializedAccount, E1014")]
    UninitializedAccount,

    #[msg("No payer present on this txn, E1015")]
    NoPayerPresent,

    #[msg("Invalid token amount, E1016")]
    InvalidTokenAmount,
}

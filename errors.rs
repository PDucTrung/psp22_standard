use ink::prelude::string::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22Error {
    Custom(String),
    InsufficientBalance,
    InsufficientAllowance,
    ZeroRecipientAddress,
    ZeroSenderAddress,
    InvalidCap,
    CapExceeded,
    SafeTransferCheckFailed(String),
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum OwnableError {
    Custom(String),
    CallerIsNotOwner,
    NewOwnerIsNotSet,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum UpgradeableError {
    Custom(String),
    SetCodeHashFailed,
    OwnableError(OwnableError),
}

impl From<OwnableError> for UpgradeableError {
    fn from(error: OwnableError) -> Self {
        UpgradeableError::OwnableError(error)
    }
}


#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AccessControlError {
    InvalidCaller,
    MissingRole,
    RoleRedundant,
}
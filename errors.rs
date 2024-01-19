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
    OwnableError(OwnableError),
    SafeTransferCheckFailed(String),
    AccessControlError(AccessControlError),
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

impl From<AccessControlError> for PSP22Error {
    fn from(error: AccessControlError) -> Self {
        PSP22Error::AccessControlError(error)
    }
}

impl From<OwnableError> for PSP22Error {
    fn from(error: OwnableError) -> Self {
        PSP22Error::OwnableError(error)
    }
}
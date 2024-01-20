use crate::OwnableError;
use ink::primitives::AccountId;

#[ink::storage_item]
#[derive(Debug, Default)]
pub struct OwnableData {
    owner: Option<AccountId>,
}

impl OwnableData {
    pub fn _init_with_owner(&mut self, account: AccountId) {
        self.owner = Some(account);
    }

    pub fn owner(&self) -> Option<AccountId> {
        self.owner
    }

    pub fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
        self.owner = None;
        Ok(())
    }

    pub fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError> {
        if new_owner == None {
            return Err(OwnableError::NewOwnerIsNotSet);
        }
        self.owner = new_owner;
        Ok(())
    }

    pub fn _check_owner(&self, account: Option<AccountId>) -> Result<(), OwnableError> {
        if self.owner != account {
            return Err(OwnableError::CallerIsNotOwner);
        }
        Ok(())
    }
}

use ink::{primitives::AccountId, storage::Mapping};

use crate::errors::AccessControlError;
use crate::traits::RoleType;
pub const DEFAULT_ADMIN_ROLE: RoleType = 0;

#[derive(Default, Debug)]
#[ink::storage_item]
pub struct AccessControlData {
    pub admin_roles: Mapping<RoleType, RoleType>,
    pub members: Mapping<(RoleType, Option<AccountId>), ()>,
}

impl AccessControlData {
    pub fn _has_role(&self, role: RoleType, address: &Option<AccountId>) -> bool {
        self.members.contains((role, address))
    }

    pub fn _add(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.members.insert((role, member), &());
    }

    pub fn _remove(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.members.remove((role, member));
    }

    pub fn _get_role_admin(&self, role: RoleType) -> Option<RoleType> {
        self.admin_roles.get(role)
    }

    pub fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        self.admin_roles.insert(role, &new_admin);
    }

    //

    pub fn _default_admin() -> RoleType {
        DEFAULT_ADMIN_ROLE
    }

    pub fn _init_with_admin(&mut self, admin: Option<AccountId>) {
        self._setup_role(AccessControlData::_default_admin(), admin);
    }

    pub fn _setup_role(&mut self, role: RoleType, member: Option<AccountId>) {
        if !self._has_role(role, &member) {
            self._add(role, &member);
        }
    }

    pub fn _do_revoke_role(&mut self, role: RoleType, account: Option<AccountId>) {
        self._remove(role, &account);
    }

    pub fn _check_role(
        &self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError> {
        if !self._has_role(role, &account) {
            return Err(AccessControlError::MissingRole);
        }
        Ok(())
    }
}

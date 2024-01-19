#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod access_control;
mod capped;
mod data;
mod errors;
mod metadata;
mod owner;
mod traits;

pub use access_control::AccessControlData;
pub use capped::Capped;
pub use data::{PSP22Data, PSP22Event};
pub use errors::{AccessControlError, OwnableError, PSP22Error, UpgradeableError};
pub use metadata::Metadata;
pub use owner::OwnableData;
pub use traits::{
    AccessControl, Ownable, PSP22Burnable, PSP22Capped, PSP22Metadata, PSP22Mintable, RoleType,
    UpgradeableTrait, PSP22,
};

#[ink::contract]
pub mod psp22_standard {
    use crate::{
        AccessControl, AccessControlData, AccessControlError, Capped, Metadata, Ownable,
        OwnableData, OwnableError, PSP22Burnable, PSP22Capped, PSP22Data, PSP22Error, PSP22Event,
        PSP22Metadata, PSP22Mintable, RoleType, UpgradeableError, UpgradeableTrait, PSP22,
    };
    use ink::prelude::{string::String, vec::Vec};

    // MINTER RoleType = 4254773782
    pub const MINTER: RoleType = ink::selector_id!("MINTER");

    #[ink(storage)]
    pub struct Psp22Standard {
        data: PSP22Data,
        metadata: Metadata,
        ownable: OwnableData,
        cap: Capped,
        admin: AccessControlData,
    }

    impl Default for Psp22Standard {
        fn default() -> Self {
            Self {
                data: Default::default(),
                metadata: Default::default(),
                ownable: Default::default(),
                cap: Default::default(),
                admin: Default::default(),
            }
        }
    }

    impl Psp22Standard {
        #[ink(constructor)]
        pub fn new(
            cap: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Self {
            let mut instance = Self::default();
            assert!(instance.cap._init_cap(cap).is_ok());
            Metadata::new(name, symbol, decimals);
            OwnableData::_init_with_owner(Self::env().caller());
            instance.admin._init_with_admin(Some(Self::env().caller()));
            instance
        }

        fn emit_events(&self, events: Vec<PSP22Event>) {
            for event in events {
                match event {
                    PSP22Event::Transfer { from, to, value } => {
                        self.env().emit_event(Transfer { from, to, value })
                    }
                    PSP22Event::Approval {
                        owner,
                        spender,
                        amount,
                    } => self.env().emit_event(Approval {
                        owner,
                        spender,
                        amount,
                    }),
                }
            }
        }
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        amount: u128,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: u128,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        old_owner: Option<AccountId>,
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }

    impl PSP22 for Psp22Standard {
        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u128 {
            self.data.balance_of(owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.data.allowance(owner, spender)
        }

        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self.data.transfer(self.env().caller(), to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .transfer_from(self.env().caller(), from, to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.data.approve(self.env().caller(), spender, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .increase_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .decrease_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP22Metadata for Psp22Standard {
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            self.metadata.token_name()
        }
        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            self.metadata.token_symbol()
        }
        #[ink(message)]
        fn token_decimals(&self) -> u8 {
            self.metadata.token_decimals()
        }
    }

    impl PSP22Mintable for Psp22Standard {
        #[ink(message)]
        fn mint(&mut self, to: AccountId, value: u128) -> Result<(), PSP22Error> {
            self.admin._check_role(MINTER, Some(to))?;
            if self.data.total_supply() + value > self.cap.cap() {
                return Err(PSP22Error::CapExceeded);
            }
            let events = self.data._mint_to(to, value)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP22Burnable for Psp22Standard {
        #[ink(message)]
        fn burn(&mut self, from: AccountId, value: u128) -> Result<(), PSP22Error> {
            self.ownable._check_owner(Some(self.env().caller()))?;
            let events = self.data._burn_from(from, value)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl Ownable for Psp22Standard {
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            self.ownable.owner()
        }
        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
            self.ownable._check_owner(Some(self.env().caller()))?;
            self.ownable.renounce_ownership()?;
            self.env().emit_event(OwnershipTransferred {
                old_owner: Some(self.env().caller()),
                new_owner: None,
            });

            Ok(())
        }
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError> {
            self.ownable._check_owner(Some(self.env().caller()))?;
            self.ownable.transfer_ownership(new_owner)?;
            self.env().emit_event(OwnershipTransferred {
                old_owner: self.owner(),
                new_owner,
            });

            Ok(())
        }
    }

    impl PSP22Capped for Psp22Standard {
        #[ink(message)]
        fn cap(&self) -> Balance {
            self.cap.cap()
        }
    }

    impl UpgradeableTrait for Psp22Standard {
        #[ink(message)]
        fn set_code(&mut self, new_code_hash: Hash) -> Result<(), UpgradeableError> {
            self.env()
                .set_code_hash(&new_code_hash)
                .map_err(|_| UpgradeableError::SetCodeHashFailed)
        }
    }

    impl AccessControl for Psp22Standard {
        #[ink(message)]
        fn has_role(&self, role: RoleType, address: Option<AccountId>) -> bool {
            self.admin._has_role(role, &address)
        }

        #[ink(message)]
        fn get_role_admin(&self, role: RoleType) -> RoleType {
            self.admin
                ._get_role_admin(role)
                .unwrap_or(AccessControlData::_default_admin())
        }
        #[ink(message)]
        fn grant_role(
            &mut self,
            role: RoleType,
            account: Option<AccountId>,
        ) -> Result<(), AccessControlError> {
            self.admin._check_role(role, account)?;
            if self.admin._has_role(role, &account) {
                return Err(AccessControlError::RoleRedundant);
            }
            self.admin._add(role, &account);
            Ok(())
        }
        #[ink(message)]
        fn revoke_role(
            &mut self,
            role: RoleType,
            account: Option<AccountId>,
        ) -> Result<(), AccessControlError> {
            self.admin._check_role(role, account)?;
            self.admin._do_revoke_role(role, account);
            Ok(())
        }
        #[ink(message)]
        fn renounce_role(
            &mut self,
            role: RoleType,
            account: Option<AccountId>,
        ) -> Result<(), AccessControlError> {
            if account != Some(Self::env().caller()) {
                return Err(AccessControlError::InvalidCaller);
            }
            self.admin._check_role(role, account)?;
            self.admin._do_revoke_role(role, account);
            Ok(())
        }
    }
}

use ink::prelude::string::String;
use crate::errors::PSP22Error;

#[ink::storage_item]
#[derive(Debug, Default)]
pub struct Capped {
    cap: u128,
}

impl Capped {
    pub fn _init_cap(&mut self, cap: u128) -> Result<(), PSP22Error> {
        if cap == 0 {
            return Err(PSP22Error::Custom(String::from("Cap must be above 0")));
        }
        self.cap = cap;
        Ok(())
    }

    pub fn cap(&self) -> u128 {
        self.cap
    }
}

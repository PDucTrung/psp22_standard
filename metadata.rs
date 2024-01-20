use ink::prelude::string::String;

#[derive(Default, Debug)]
#[ink::storage_item]
pub struct Metadata {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u8,
}

impl Metadata {
    pub fn token_name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn token_symbol(&self) -> Option<String> {
        self.symbol.clone()
    }

    pub fn token_decimals(&self) -> u8 {
        self.decimals
    }
}

use ink::prelude::string::String;

#[derive(Default, Debug)]
#[ink::storage_item]
pub struct Metadata {
    name: Option<String>,
    symbol: Option<String>,
    decimals: u8,
}

impl Metadata {
    pub fn new(name: Option<String>, symbol: Option<String>, decimals: u8) -> Metadata {
        let mut data = Metadata {
            name: Default::default(),
            symbol: Default::default(),
            decimals: Default::default(),
        };
        data.name = name;
        data.symbol = symbol;
        data.decimals = decimals;
        data
    }

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

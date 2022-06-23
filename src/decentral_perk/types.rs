use candid::{CandidType, Principal};
use serde::{Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
}

impl Default for Vendor {
    fn default() -> Self {
        Self {
            principal_id: Principal::anonymous(),
            name: Default::default(),
            description: Default::default(),
            website: Default::default(),
            products: Default::default()
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Vendor {
    pub principal_id: Principal,
    pub name: String,
    pub description: String,
    pub website: String,
    pub products: Option<Vec<Product>>
    // TODO: Add ICP address, BTC address, DIP20 address
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Product {
    pub principal: Principal,
    pub name: String,
    // Add photo, description, price
}

pub type GetVendorReceipt = Result<Vendor, GetVendorErr>;

#[derive(CandidType)]
pub enum GetVendorErr {
    NoStoreFound
}

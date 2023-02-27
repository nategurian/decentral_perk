use std::collections::BTreeMap;

use ic_cdk::export::{
    candid::{CandidType, Principal},
    serde::Deserialize
};

use crate::types;

#[derive(CandidType, Deserialize, Default)]
pub struct State {
    pub users: BTreeMap<Principal, types::User>,
    pub posts: BTreeMap<Principal, types::Post>
}

impl State {
    pub fn new() -> Self {
        Self {
            users: BTreeMap::new(),
            posts: BTreeMap::new()
        }
    }
}
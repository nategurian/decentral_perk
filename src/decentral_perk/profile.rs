use candid::{Principal};
use ic_cdk_macros::*;
use ic_cdk::{caller, println};
use std::cell::RefCell;
use std::collections::BTreeMap;
use crate::types::{Profile, Vendor, Product};

type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;
type VendorStore = BTreeMap<Principal, Vendor>;

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
    static VENDOR_STORE: RefCell<VendorStore> = RefCell::default();
}

#[query(name = "getSelf")]
fn get_self() -> Profile {
    let id = caller();
    PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&id)
            .cloned()
            .unwrap_or_else(|| Profile::default())
    })
}

#[query]
fn get(name: String) -> Profile {
    ID_STORE.with(|id_store| {
        PROFILE_STORE.with(|profile_store| {
            id_store
                .borrow()
                .get(&name)
                .and_then(|id| profile_store.borrow().get(id).cloned())
                .unwrap_or_else(|| Profile::default())
        })
    })
}

#[update]
fn update(profile: Profile) {
    let principal_id = caller();
    ID_STORE.with(|id_store| {
        id_store
            .borrow_mut()
            .insert(profile.name.clone(), principal_id);
    });
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(principal_id, profile);
    });
}

#[update(name="setVendor")]
fn set_vendor(vendor: Vendor) {
    VENDOR_STORE.with(|vendor_store| {
        vendor_store
            .borrow_mut()
            .insert(vendor.principal_id, vendor);
    });
}

#[query(name="getVendorById")]
fn get_vendor_by_id(principal_id: Principal) -> Vendor {
    return VENDOR_STORE.with(|vendor_store| {
        vendor_store
            .borrow()
            .get(&principal_id)
            .cloned()
            .unwrap_or_else(|| Vendor{
                principal_id: principal_id,
                name: "".to_string(),
                description: "".to_string(),
                website: "".to_string(),
                products: Some(vec![])
            })
    });
}

#[query(name="getMyStore")]
fn get_my_store() -> Vendor {
    let id = caller();
    return VENDOR_STORE.with(|vendor_store| {
        vendor_store
            .borrow()
            .get(&id)
            .cloned()
            .unwrap_or_else(|| Vendor{
                principal_id: id,
                name: "".to_string(),
                description: "".to_string(),
                website: "".to_string(),
                products: Some(vec![])
            })
    });
}

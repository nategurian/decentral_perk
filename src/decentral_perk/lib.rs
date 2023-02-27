mod state;
mod types;
mod utils;
mod api;

use std::cell::RefCell;
use candid::candid_method;
use ic_cdk_macros::{post_upgrade, pre_upgrade, update, query};
use ic_cdk::export::Principal;
use state::State;
use types::{AddPostRequest, AddPostResponse, Post};

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::new(Some(State::new()));
}

/// A helper method to read the state.
///
/// Precondition: the state is already initialized.
pub fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow().as_ref().expect("state not initialized")))
}

// A helper method to mutate the state.
//
// Precondition: the state is already initialized.
pub fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(cell.borrow_mut().as_mut().expect("state not initialized")))
}

// A helper method to set the state.
//
// Precondition: the state is _not_ initialized.
fn set_state(state: State) {
    STATE.with(|cell| {
        // Only assert that the state isn't initialized in production.
        // In tests, it is convenient to be able to reset the state.
        #[cfg(target_arch = "wasm32")]
        assert!(
            cell.borrow().is_none(),
            "cannot initialize an already initialized state"
        );
        *cell.borrow_mut() = Some(state)
    });
}

//// LEAVE FOR NOW, we may want to initialize a state later on.
// #[init]
// pub fn init() {
//     // let init_at = utils::time_secs();
//     // let payload = maybe_payload.expect("There should be a payload when initializing the canister.");
//     // assert!(
//     //     payload.seed_admin_principal_id.is_some(),
//     //     "Seed principal ID should be set when initializing the canister."
//     // );
//     // assert!(
//     //     payload.asset_canister_id.is_some(),
//     //     "Asset canister ID should be set when initializing the canister."
//     // );
// // 
//     // let payload = InitPayload::with_init_at(payload, init_at);
//     // ic_cdk::println!("{:#?}", payload);
//     set_state(State::new())
// }

#[pre_upgrade]
pub fn pre_upgrade() {
    with_state(|state| ic_cdk::storage::stable_save((state,))).expect("Saving state must succeed.")
}

// #[post_upgrade]
// pub fn post_upgrade() {
//     let state = ic_cdk::storage::stable_restore::<(State,)>()
//         .expect("Failed to read from stable memory.")
//         .0;
//     set_state(state);
// }

#[update]
#[candid_method(update)]
pub async fn add_post(request: AddPostRequest) -> AddPostResponse {
    api::add_post(request)
}

#[query]
#[candid_method(query)]
pub async fn get_posts() -> Vec<(Principal, Post)> {
    api::get_posts()
}

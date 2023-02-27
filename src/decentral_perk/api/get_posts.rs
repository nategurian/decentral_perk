use std::borrow::Borrow;

use crate::{types::{Post},
 with_state, utils};

use ic_cdk::export::Principal;

/// Retrieves all the posts in the state.
pub fn get_posts() -> Vec<(Principal, Post)> {
  with_state(|state| {
    state
        .posts
        .iter()
        .map(|(k, post)| (k.clone(), post.clone()))
        .collect()
  })
}
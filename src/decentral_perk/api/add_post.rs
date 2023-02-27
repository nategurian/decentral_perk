use ic_cdk::caller;
use crate::{types::{AddPostResponse, AddPostRequest, Post, AddPostResponseType},
 with_state_mut, utils};

/// Adds a post to the post collection.
pub fn add_post(request: AddPostRequest) -> AddPostResponse {
    let caller = caller();
    with_state_mut(|state| {
        let created_at = utils::time_secs();

        state
            .posts
            .insert(caller, Post{
              created_at,
              content: request.content,
              category: request.category,
              likes: 0
            });

        AddPostResponse {
          response: AddPostResponseType::Success
        }
    })
}
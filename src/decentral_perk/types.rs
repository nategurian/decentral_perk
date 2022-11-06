use ic_cdk::export::{
    candid::{CandidType},
    serde::Deserialize
};

#[derive(CandidType, Deserialize)]
pub struct User {
    pub username: String
}

#[derive(CandidType, Deserialize)]
pub struct AddUserRequest {
    pub username: String
}

#[derive(CandidType, Deserialize)]
pub struct AddUserResponse {
    pub response: AddUserResponseType
} 

#[derive(CandidType, Deserialize)]
pub enum AddUserResponseType {
    Success,
    AlreadyAUser,
    Error
}

#[derive(CandidType, Deserialize)]
pub struct Post {
    pub created_at: u64,
    pub content: String,
    pub category: PostCategory
}

#[derive(CandidType, Deserialize)]
pub enum PostCategory {
    General,
    BrewTip,
    Roasts
}

#[derive(CandidType, Deserialize)]
pub struct AddPostRequest {
    pub content: String
}

#[derive(CandidType, Deserialize)]
pub struct AddPostResponse {
    pub response: AddUserResponseType
} 

#[derive(CandidType, Deserialize)]
pub enum AddPostResponseType {
    Success,
    Error
} 
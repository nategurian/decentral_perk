import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AddPostRequest { 'content' : string }
export interface AddPostResponse { 'response' : AddPostResponseType }
export type AddPostResponseType = { 'Error' : null } |
  { 'Success' : null };
export interface AddUserRequest { 'username' : string }
export interface AddUserResponse { 'response' : AddUserResponseType }
export type AddUserResponseType = { 'Error' : null } |
  { 'Success' : null } |
  { 'AlreadyAUser' : null };
export interface Post { 'content' : string, 'created_timestamp' : bigint }
export interface User { 'username' : string }
export interface _SERVICE {
  'add_post' : ActorMethod<[AddPostRequest], AddPostResponse>,
  'add_user' : ActorMethod<[AddUserRequest], AddPostResponse>,
}

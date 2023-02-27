export const idlFactory = ({ IDL }) => {
  const PostCategory = IDL.Variant({
    'General' : IDL.Null,
    'BrewTip' : IDL.Null,
    'Roasts' : IDL.Null,
  });
  const AddPostRequest = IDL.Record({
    'content' : IDL.Text,
    'category' : PostCategory,
  });
  const AddPostResponseType = IDL.Variant({
    'Error' : IDL.Null,
    'Success' : IDL.Null,
  });
  const AddPostResponse = IDL.Record({ 'response' : AddPostResponseType });
  const AddUserRequest = IDL.Record({ 'username' : IDL.Text });
  const Post = IDL.Record({
    'content' : IDL.Text,
    'created_at' : IDL.Nat64,
    'likes' : IDL.Nat64,
    'category' : PostCategory,
  });
  return IDL.Service({
    'add_post' : IDL.Func([AddPostRequest], [AddPostResponse], []),
    'add_user' : IDL.Func([AddUserRequest], [AddPostResponse], []),
    'get_posts' : IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Principal, Post))], []),
  });
};
export const init = ({ IDL }) => { return []; };

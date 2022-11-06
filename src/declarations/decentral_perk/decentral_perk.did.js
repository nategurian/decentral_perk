export const idlFactory = ({ IDL }) => {
  const AddPostRequest = IDL.Record({ 'content' : IDL.Text });
  const AddPostResponseType = IDL.Variant({
    'Error' : IDL.Null,
    'Success' : IDL.Null,
  });
  const AddPostResponse = IDL.Record({ 'response' : AddPostResponseType });
  const AddUserRequest = IDL.Record({ 'username' : IDL.Text });
  return IDL.Service({
    'add_post' : IDL.Func([AddPostRequest], [AddPostResponse], []),
    'add_user' : IDL.Func([AddUserRequest], [AddPostResponse], []),
  });
};
export const init = ({ IDL }) => { return []; };

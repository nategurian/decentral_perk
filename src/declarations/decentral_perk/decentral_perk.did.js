export const idlFactory = ({ IDL }) => {
  const Profile_2 = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'keywords' : IDL.Vec(IDL.Text),
  });
  const Vendor = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'website' : IDL.Text,
  });
  const GetVendorErr = IDL.Variant({ 'NoStoreFound' : IDL.Null });
  const GetVendorReceipt = IDL.Variant({ 'Ok' : Vendor, 'Err' : GetVendorErr });
  return IDL.Service({
    'get' : IDL.Func([IDL.Text], [Profile_2], ['query']),
    'getMyStore' : IDL.Func([], [GetVendorReceipt], ['query']),
    'getSelf' : IDL.Func([], [Profile_2], ['query']),
    'update' : IDL.Func([Profile_2], [], []),
  });
};
export const init = ({ IDL }) => { return []; };

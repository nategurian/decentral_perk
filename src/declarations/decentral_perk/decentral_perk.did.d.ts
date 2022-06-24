import type { Principal } from '@dfinity/principal';
export type GetVendorErr = { 'NoStoreFound' : null };
export type GetVendorReceipt = { 'Ok' : Vendor } |
  { 'Err' : GetVendorErr };
export type Profile = Profile_2;
export interface Profile_2 {
  'name' : string,
  'description' : string,
  'keywords' : Array<string>,
}
export interface Vendor {
  'name' : string,
  'description' : string,
  'website' : string,
}
export interface _SERVICE {
  'get' : (arg_0: string) => Promise<Profile_2>,
  'getAllVendors' : () => Promise<Array<Vendor>>,
  'getMyStore' : () => Promise<GetVendorReceipt>,
  'getSelf' : () => Promise<Profile_2>,
  'update' : (arg_0: Profile_2) => Promise<undefined>,
}

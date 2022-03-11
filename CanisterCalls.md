# Command Line Calls

## Profile Calls

### getSelf
```
dfx canister call decentral_perk getSelf '()'
```

### get
```
dfx canister call decentral_perk get '("hello")'
```

### update
```
dfx canister call decentral_perk update '(record {name = "Nate"; description = "Creator of decentral_perk"; keywords = vec {"developer"; "engineer"}})'
```

### setVendor
```
dfx canister call decentral_perk setVendor '(record {name = "Startbucks"; description = "Largest coffee retailer inthe world"; website = "www.starbucks.com"})'
```


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
 export PRINCIPAL=jkjgp-5hz3y-nunep-6fvoj-ojiml-4sryr-4wfmr-kohxn-6evd6-g5r6l-mae
dfx canister call decentral_perk setVendor '(record {principal_id = principal '\"$PRINCIPAL\"'; name = "Startbucks"; description = "Largest coffee retailer in the world"; website = "www.starbucks.com"})'
```


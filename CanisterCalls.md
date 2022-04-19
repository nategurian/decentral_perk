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
 export PRINCIPAL=7r6l7-6nyo7-dr7ty-6t7ea-aocur-lcmhl-vlgsa-pzcxt-tl3rq-imi3v-qqe
dfx canister call decentral_perk setVendor '(record {principal_id = principal '\"$PRINCIPAL\"'; name = "Startbucks"; description = "Largest coffee retailer in the world"; website = "www.starbucks.com"})'
```

### getVendorById
```
 export PRINCIPAL="principal \"$(dfx identity get-principal)\""
 dfx canister call decentral_perk getVendorById "($PRINCIPAL)"
```


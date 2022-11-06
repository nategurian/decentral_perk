# Command Line Calls

## Post calls

### post
```
export PRINCIPAL = fdfuh-sxpv4-pe7jk-dmr6w-ttsdo-fowjx-uragx-nnmt6-c3z4x-udpwn-qae
dfx canister call decentral_perk post 'record {title = "First Post"; content = "This is the first post!"; category = "General"}'
```

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
 export PRINCIPAL=fdfuh-sxpv4-pe7jk-dmr6w-ttsdo-fowjx-uragx-nnmt6-c3z4x-udpwn-qae
dfx canister call decentral_perk setVendor '(record {principal_id = principal '\"$PRINCIPAL\"'; name = "Nates Cafe"; description = "Specialty Roastery"; website = "www.natesCafe.com"})'
```

### getVendorById
```
 export PRINCIPAL="principal \"$(dfx identity get-principal)\""
 dfx canister call decentral_perk getVendorById "($PRINCIPAL)"
```


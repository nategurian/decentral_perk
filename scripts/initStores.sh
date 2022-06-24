#!/bin/bash

NAMES=("Nates Cafe" "Starbucks" "Buddy Brew")
DESCRIPTIONS=("Locally owned roastery."
  "Largest Coffee Retailer In the world."
  "Florida Based coffee retailer.")

WEBSITES=("www.natescafe.com"
  "www.starbucks.com"
  "www.buddybrew.com") 

for i in ${!NAMES[@]}; do
  # create a new identity for principal creation
  dfx identity new $i
  dfx identity use $i
  PRINCIPAL=$(dfx identity get-principal)
  NAME=NAMES[$i]
  DESCRIPTION=DESCRIPTIONS[$i]
  WEBSITE=WEBSITES[$i]
  #constrcut DFX call/object
  echo "Creating Vendor record for ${PRINCIPAL}"
  dfx canister call decentral_perk setVendor '(record {principal_id = principal '\"$PRINCIPAL\"'; name = "'"${NAMES[$i]}"'"; description = "'"${DESCRIPTIONS[$i]}"'"; website = "'"${WEBSITES[$i]}"'"})'
done

echo "Finished initialzing vendors"
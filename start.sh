dfx start --clean --background

echo 'deploying backend'
dfx deploy decentral_perk

echo 'installing frontend'
dfx deploy decentral_perk_assets

echo 'installing internet identity'
II_ENV=development dfx deploy --no-wallet --argument '(null)'
dfx canister call internet_identity init_salt
echo $?

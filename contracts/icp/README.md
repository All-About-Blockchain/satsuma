# ICP Yield Vault Canister (Rust)

## Build and Deploy Locally

```sh
dfx start --background
dfx deploy
```

## Deposit Yield

```sh
dfx canister call icp_yield_vault deposit_yield '(1000000:nat64)'
```

## Query Balance

```sh
dfx canister call icp_yield_vault get_balance '("<principal>")'
dfx canister call icp_yield_vault my_balance
```

## Notes
- The canister tracks balances per principal.
- Only the caller's balance is incremented on deposit.
- Integrate with Axelar GMP or a relayer to call `deposit_yield` when bridging from Injective. 
# Backend exists at - https://github.com/h-wal/vault_backend
# Collateral Vault Program (Anchor / Solana)

Smart contract that manages tokenized collateral vaults on Solana. Each vault is a PDA-owned token account that tracks total, locked, and available balances while enforcing controlled transfers through an authority module.

## Program ID

- Update `Anchor.toml` and redeploy if you change the program ID.

## Accounts / State

- `CollateralVault`: stores `owner`, `token_account`, `mint`, `total_balance`, `locked_balance`, `available_balance`, cumulative `total_deposited` / `total_withdrawn`, `created_at`, and `bump`. Invariant: `total_balance = locked_balance + available_balance`.
- Vault PDA: derived for each owner/mint pair (see `initialize_vault`); owns the token account holding collateral.
- Vault Authority: PDA that gates privileged calls and maintains an allowlist of authorized programs (see `vault_authority` module).

## Instructions

- `initialize_vault`: create and configure a new vault PDA and its token account for an owner/mint pair.
- `deposit(amount)`: transfer SPL tokens into the vault; updates totals and available balance.
- `lock_collateral(amount)`: move funds from available to locked for downstream protocols.
- `unlock_collateral(amount)`: return locked funds to available.
- `withdraw(amount)`: transfer available funds back to the owner; updates totals.
- `initialize_vault_authority`: set up the authority PDA for managing program allowlists.
- `add_authorized_program(program_id)`: extend the allowlist of CPI-capable programs.
- `transfer_collateral(amount)`: controlled movement of collateral under authority checks.

## Build & Test

```bash
# from repo root
anchor build
anchor test
```

## Deployment

```bash
anchor keys list           # confirm program key
anchor deploy              # deploy and write ID to target/
# then update Anchor.toml and re-run clients with the new ID if needed
```

## Security Notes

- All state transitions are PDA-gated; ensure the program ID and seeds match between client and on-chain program.
- Balance invariants are enforced per instruction; client-side code should still validate amounts before submission.
- Use the vault authority allowlist to restrict which programs can perform CPI-based collateral movements.

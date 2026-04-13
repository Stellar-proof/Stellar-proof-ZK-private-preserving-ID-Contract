
#  IdentityContract — ZK Identity Verification (Soroban)

A Soroban smart contract that verifies zero-knowledge identity claims generated off-chain using a Noir circuit.

It enables users to prove identity ownership and backend authorization **without revealing any private identity data on-chain**.

---

#  Overview

This contract is part of a privacy-preserving identity system built on Stellar Soroban.

Instead of submitting raw identity data or signatures, users submit a **zero-knowledge proof** that verifies:

- They own a valid identity commitment
- They have a valid  authorization
- They have not reused the same claim (nullifier protection)
- They satisfy a given condition (e.g. `"OVER18"`)

Only the proof and public inputs are verified on-chain.

---

#  Key Features

-  Zero-knowledge identity verification
-  Commitment-based identity storage
-  Replay protection using nullifiers
-  Condition-based claims (e.g. age gating)
-  signed authorization support
-  Designed for Soroban smart contracts

---

#  Storage Layout

## Identity Commitment

Stores the hashed identity for each user:

```rust
DataKey::Identity(Address) -> BytesN<32>
````

---

## Trusted Signer

Stores backend public key used to authorize identity claims:

```rust
DataKey::Signer -> BytesN<32>
```

---

## Nullifier Registry

Prevents replay attacks:

```rust
DataKey::UsedNullifier(BytesN<32>) -> bool
```

---

#  Main Function

## verify_claim

```rust
fn verify_claim(
    user: Address,
    identity_hash: BytesN<32>,
    nullifier: BytesN<32>,
    condition: Bytes,
    proof: Bytes
) -> bool
```

---

#  Verification Logic

The contract performs three core checks:

---

## 1. Identity Validation

Ensures the user’s identity matches the stored commitment:

```rust
require!(get_identity(user) == identity_hash);
```

---

## 2. Nullifier Check (Replay Protection)

Prevents reuse of the same claim:

```rust
require!(!is_nullifier_used(nullifier));
mark_nullifier_used(nullifier);
```

---

## 3. Zero-Knowledge Proof Verification

Verifies the ZK proof that binds all inputs together:

* identity_hash
* nullifier
* user_address
* condition
* signer_pubkey
* backend signature (inside proof)

```rust
verify_zk_proof(proof, public_inputs)
```

---

#  Signed Message Format

The backend signs the following canonical message:

```text
user_address || identity_hash || nullifier || condition
```

This ensures:

* no tampering of identity
* no condition manipulation
* no replay attacks
* correct user binding

---

#  Security Guarantees

✔ No identity data is exposed on-chain
✔ Backend signature is enforced via ZK proof
✔ Nullifiers prevent double-spending
✔ Identity commitment cannot be forged
✔ Condition cannot be modified after signing

---

# ⚙️ Public Inputs

| Name            | Type       | Description                   |
| --------------- | ---------- | ----------------------------- |
| `identity_hash` | BytesN<32> | Stored identity commitment    |
| `nullifier`     | BytesN<32> | Replay protection hash        |
| `user_address`  | Address    | Claiming user                 |
| `signer_pubkey` | BytesN<32> | Trusted backend key           |
| `condition`     | Bytes      | Claim condition (e.g. OVER18) |

---

#  Flow

```text
1. User registers identity → identity_hash stored
2. Backend signs authorization message
3. User generates ZK proof off-chain
4. User submits proof + public inputs
5. Contract verifies proof
6. Nullifier is marked as used
7. Claim is approved
```

---

#  Trust Model

* ✔ Trusted: backend signer
*  Not trusted: user inputs, frontend, proof generator
* ✔ Trust minimized via ZK proofs

---

# 🔗 Architecture

```text
User
 ↓
Noir ZK Circuit (proves validity)
 ↓
ZK Proof
 ↓
Soroban IdentityContract
 ↓
On-chain verification result
```

---

#  Use Cases

* Age-restricted access (OVER18 / OVER21)
* Private identity verification
* Sybil resistance systems
* DAO membership gating
* Private whitelist claims
* Token airdrop eligibility

---

#  Notes

* Nullifiers must be unique per claim
* Identity hashes must be pre-registered
* Backend signer must be securely managed
* Proof verification depends on circuit correctness

---

#  License

MIT 

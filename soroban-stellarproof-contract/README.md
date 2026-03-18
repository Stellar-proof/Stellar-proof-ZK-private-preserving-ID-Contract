# Privacy-Preserving Identity Contract (Soroban)

## 📌 Overview

This contract implements a **privacy-preserving identity verification system** on the Stellar using Soroban.

It allows users to:

* Register a **hashed identity commitment**
* Prove properties about their identity (e.g., age, nationality)
* Without revealing sensitive personal data

---

## 🧠 Design Philosophy

Instead of storing raw identity data on-chain, this system uses:

* **Commitments (hashes)** → represent identity
* **Zero-Knowledge Proofs (ZKPs)** → prove claims privately
* **Off-chain verification** → reduce on-chain complexity
* **Signature validation** → ensure integrity of verified proofs

---

## 🏗️ Architecture

### Components

| Layer    | Responsibility                                   |
| -------- | ------------------------------------------------ |
| Circuit  | Generates ZK proofs from private identity data   |
| Backend  | Verifies documents + generates & verifies proofs |
| Contract | Stores commitments + verifies signed claims      |

---

##  Identity Model

Each identity is normalized into:

```
name
unique_number
dob
state
country
doc_type
secret
```

A commitment is generated:

```
identity_hash = Poseidon(...)
```

Only `identity_hash` is stored on-chain.

---

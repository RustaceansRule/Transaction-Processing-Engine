# Transaction Processing Engine

## Overview

This project implements a **transaction processing engine** for a simple financial system. It handles multiple clients’ accounts and supports:

- Deposits  
- Withdrawals  
- Disputes  
- Resolves  
- Chargebacks  

The engine is designed to **process CSV data efficiently** and can be extended to handle data from other sources (e.g., TCP streams) in the future.

This is not a production ready implementation and could be improved if needed to be scaled etc
---

## Features

- **Accounts Management:** Tracks `available`, `held`, `total` funds and account `locked` status.  
- **Transaction Tracking:** Maintains minimal state for transactions to handle disputes, resolves, and chargebacks.  
- **Error Handling:** Supports `InsufficientFunds` and `AccountLocked` errors.  
- **Concurrency Ready:** Uses asynchronous channels (`tokio::mpsc`) for ingesting data, suitable for streaming sources.  

---

## CSV Input Format

The CSV must include the following columns:

- **type:** `deposit`, `withdrawal`, `dispute`, `resolve`, `chargeback` (case-insensitive).  
- **client:** Unique client ID (integer).  
- **tx:** Global unique transaction ID (integer).  
- **amount:** Decimal amount (required for deposits/withdrawals; empty for disputes, resolves, chargebacks).  

### Example
```csv 
type,client,tx,amount
deposit,1,1,100.0
withdrawal,1,2,50.0
dispute,1,1,
resolve,1,1,
chargeback,1,1,
```


---

## Account Behavior Rules

| Transaction | Effect on Account |
|------------|-----------------|
| Deposit    | `available += amount`, `total += amount` |
| Withdrawal | `available -= amount`, `total -= amount` (error if insufficient funds) |
| Dispute    | `available -= amount`, `held += amount` (funds held, total unchanged) |
| Resolve    | `held -= amount`, `available += amount` (total unchanged) |
| Chargeback | `held -= amount`, `total -= amount`, account locked |

---

## Code Decisions

- **Data Structures:**  
  - `HashMap<u16, Account>` for accounts → efficient lookup by client ID.  
  - `HashMap<u32, Transaction>` for transactions → stores transactions as any transaction can be disputed in the future, we don't know when
- **Zero-Copy Considerations:**  
  - Currently, CSV fields are deserialized into `InputData`. Using `serde::Deserialize` String possibles are stared as enum type. No checks on invalid variant being present in the csv at this stage.  
  - Transactions only store amounts and dispute flags, reducing heap usage.  
- **Concurrency:**  
  - `tokio::mpsc` channel is used to decouple data ingestion from processing. This design can scale to thousands of simultaneous streams.  
- **Error Handling:**  
  - Invalid transactions (non-existent disputes, resolves, or chargebacks) are ignored, as per spec.
  - Errors where emitted are bubbled or not report for the purpose allowing the full processing of the file, assuming the csv file is valid and correctly formatted, no validation as such if done on this, "Amount" field is marked as optional can be missing for some transaction types.

---

## Expected Output

After processing a CSV file, the engine prints account summaries, Example:

```csv
client,available,held,total,locked
1,20,0,20,false
2,24,0,24,false
3,21,7,28,false
4,0,0,0,true
5,27,9,36,false
6,40,0,40,false
7,11,0,11,true
8,48,0,48,false
9,26,0,26,true
10,42,14,56,false
```


- `available + held = total` (except when chargebacks reduce total)  
- Accounts are `locked` after chargebacks.  
- Held funds represent unresolved disputes.

---

## Tests

- Unit tests cover:
  - Deposit and withdrawal behavior  
  - Dispute, resolve, and chargeback behavior  
  - Account locking and insufficient funds handling  
- Test CSV data includes:
  - Multiple clients (1–10)  
  - 500 transactions  
  - All possible transaction types  

---

## Possible Improvements

1. **Zero-Copy CSV Parsing:** Use `csv::ByteRecord` with `serde::Deserialize` to reduce string allocations.  
2. **Custom Data Sources:** Support streaming sources (TCP, Kafka, etc.) using the existing `tokio::mpsc` design.  
3. **Metrics & Monitoring:** Track processing speed, memory usage, and dispute/chargeback counts for auditing.
4. **Error reporting** Possibly via a separate channel for external logging or UI display 

---

## Usage

``` bash 
cargo run -- transactions.csv > accounts.csv
```

## License

This project is MIT licensed.
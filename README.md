# Transaction Processor 

## Description
This library takes an input csv file representing client transactions. The output is a csv file with columns representing the client's state of account (client, available, held, total, locked).

## Building the tool

```bash
cargo build -- release
```

## Running the tool

```bash
cargo run -- transactions.csv > client_accounts.csv
```

## Runnign unit tests

```bash
cargo test
```

## Assumptions
1) If account is locked, the client cannot make deposits or withdrawls

2) Once the disputed transaction has been resolved, the resolution of this transaction cannot happen the second time. 

## Design
* Data structure for client account

A hashmap is chosen to store the client's state of account. This is a convinient structure because the client's id can be used as the map's key, while the the struct representing the actual account as the value. 

* Data structure for storing transaction

The transactions for deposit and withdrawl need to be stored in order to handle dispute, resolve and chargeback transactions. A hashmap data structure is chosen to store transaction history. The key is a transaction id (tx) because it is globally unique, while the value is a struct contaning two fields: client id and the amount value. While we do not need to store client id, because the transaction id is globally unique, it is still stored for sanity check. 

* Reading the CSV file

The csv crate was chosen as a convinient way for reading the csv and writting into stdout. Because the csv file may contain a large number of rows, we first allocate memory for a single row using ByteRecord and reuse it for each row instead of reading the whole file into memory at once and allocating memory for each row. Thus, we can reuse the allocated memory and achieve amortized allocation.

* Potential Improvements

Another potential optimization is implementing an async csv reader in case we are reading from multiple TCP streams. It is possible to harness the power of I/O concurrency using the async-std or the tokio crates where multiple threads share the state of transactions and client accounts (probably by using a mutex lock and unlock). However, it was decided to only mention it as a potential future improvemement of the tool as more personal research of the topic needs to be done. 




# Transaction Processor
This project is a simple transaction processor which consumes data from input file (.csv) or from stdin.
Takes in a bunch of transactions of users and computes the final state for all users.

## Usage
You can compile code and from root folder execute any of the following command:<br>
`$ cargo run -- example.csv` > output.csv`<br>
`$ cat example.csv | cargo run > output.csv`<br>
where `example.csv` is valid csv file with data<br> 

## Technical Design
Technical considerations were carried out in `docs/TDD.md`.
A simple transaction processor utility


## Highlights

Decimal- important to deal with floating point eccentricities (paramount when dealing with money)
Precision - 4 places after decimal point (gets rounded depending on the 5th digit after zero)
Rejects negative amounts
If deposit/resolve/chargeback's client is different from the client of the referred tx, such tx is considered erroneous and doesn't get processed
Frozen/locked account never get unlocked; presumably needs to be inspected by a human agent
Deposits, withdrawals and chargebacks are not possible for frozen accounts, but it's possible to file a dispute and resolve that dispute
I/O, parsing and other errors get propagated and printed out and the program exits
It's possible to end up with negative balance (deposit -> withdraw -> dispute [-> chargeback]); such person is considered to owe money to the system owner
Unit tests in main.rs and integration tests in tests/
Disputing an already disputed tx does nothing
It's not possible to dispute a withdrawal as it's not compatible with the specs
Throughput is about 25000 tx/s (2,9 GHz Quad-Core Intel Core i7; 2133 MHz LPDDR3)
                                                
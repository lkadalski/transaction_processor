# Transaction Processor
This project is a simple transaction processor which consumes data from input file (.csv) or from stdin.
Takes in a bunch of transactions of users and computes the final state for all users.

## Usage
You can compile code and from root folder execute any of the following command:<br>
`$ cargo run -- example.csv` > output.csv`<br>
`$ cat example.csv | cargo run > output.csv`<br>
where `example.csv` is valid csv file with data<br> 
Verbose output with logs: <br>
`$ RUST_LOG=info cargo run -- example.csv` > output.csv`<br>

## Technical Design
Technical considerations were carried out in `docs/TDD.md`.
A simple transaction processor utility


## Highlights

* Decimal - important to deal with floating point eccentricities (paramount when dealing with money)<br>
* Precision - 4 places after decimal point (gets rounded depending on the 5th digit after zero)<br>
* Rejects negative amounts for withdrawal and deposits<br>
* If deposit/resolve/chargeback's client is different from the client of the referred tx, such tx is considered erroneous and doesn't get processed <br>
* Frozen/locked account never get unlocked; presumably needs to be inspected by a human agent <br>
* Deposits, withdrawals and chargebacks are not possible for frozen accounts, but it's possible to file a dispute and resolve that dispute <br>
* I/O, parsing and other errors get propagated and printed out and the program exits. Beside async critial errors (like failing to read/wrtie from channel)<br>
* Unit tests in mod.rs and integration tests in tests/<br>
* Disputing an already disputed tx does nothing <br>
* Dispute is possible for both withdrawal and deposit <br>
                                                

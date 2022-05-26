# Technical Design Document

### Author: lkadalski
### Created Date: 2022-05-26

## Overview
This is design document for `transcation_processor` application. Main goal is create program which consumes arbitrary CSV file and process it's records.
Typical user should be able to:
* Process data with input file and have output with summary
* Have meaningful message when something is wrong with file(Character encoding, wrong file format, data corrupted, OOM?)
* Use program through CLI with only one argument (file)
* Use program for big files (records up to u32::MAX(4kkk))

> Implementation should be carried out with Rust langauge. <br>

## Proposed solution

### CLI

This program should accept only one argument which is a file path. Nevertheless there should be a way to increase verbosity of a program.
This could be achived by running program with env variable `RUST_LOG=DEBUG` to allow debug level logs be printed in std out.

In first step program should verify whether file exists and is in valid UTF-8 format.
To save computer resources in second step an async runtime should be instantiated.
Two tasks should be created: FileReader, RecordProcessor and DataPrinter?(async-csv).
Data should be keep in HashMap, with a Key as accountId.


`TODO` -> Verify if typical stack is able to hold u32::MAX records in memory

* Prepare typical data set(or script which will generate those)
* Provide automated tests
* Provide unit tests

### Example usage:

* Typical use case <br>
`$ transaction_processor input.csv` <br>

```
client,available,held,total,locked
2,2,0,2,false
1,1.5,0,1.5,false
```

### Transaction Processor

Main code should be wrapped with `async` runtime and utilise Async Pipeline design pattern. <br>
There should be such components:
* `FileReader` - which reads file line by line and pushes record to channel
* `RecordProcessor` - which process data and updates specific data in DataStorage. 
* `DataPrinter` - which prints the data record by record in an async way


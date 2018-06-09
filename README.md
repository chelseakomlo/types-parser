## Tool to turn Rust types into C

### About

Currently only supports converting Rust constants into C constants. Later this
could be extended to structs and/or enums.

To run:
`cargo run <intputfile_generated.rs> <outputfile_generated.h>`

For example input and output files, see `/docs`.

All Rust files to be generated must end in `_generated.rs` and consequently all
C files that will be the output will also end in `_generated.h`

### Convention

Rust files should be logically grouped by domain. For example, separate files
should exist for constants related to hidden services than for bridges.

Files should be named according to both the domain the file is for, and also
to specify that the file is coupled to c.


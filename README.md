# elba - a package manager for Idris
## Installation
`elba` is written in Rust, so the Cargo and the Rust compiler are required for building.

After those are installed, clone this repo and whack `cargo install`.

## Testing
One note for testing is that the integration tests will lock folders in the `data/` directory, since in a real-life application you don't want multiple instances of `elba` clobbering each others' work directory, so make sure to pass `--test-threads 1` to the test binary.
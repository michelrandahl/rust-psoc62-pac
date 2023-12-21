# rust-psoc62-pac


## How the PAC was generated

1. generate PAC with svd2Rust
    * svd2rust -i xxx.svd
2. rustfmt lib.rs
3. cargo init --lib psoc62-pac
    * mkdir src
4. cargo check --target thumbv7em-none-eabihf
5. got some errors with duplicate `FM_CTL`, renamed one of them to `FM_CTL_REG` which resolved the issue.
6. run svd2rust again to generate new lib.rs
7. explode the lib.rs file with form
    - `cargo install form`
    - `form -i lib.rs -o ./src/`
    - `cargo fmt`

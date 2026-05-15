# Flash Algorithm for the MakerPnPControl-CORE board

This is a flash algorithm template for writing CMSIS-Pack flash algorithms in Rust.
It can be used to flash the external flash chip on the board using `probe-rs`.

## Current status

Incomplete, not usable yet due to issues:

```
Message: Init
PWR::CR3 = 0x00010044
RCC::AHB4ENR = 0x00000000
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Erase sector addr: 0x70000000
Physical addr: 0x00000000
Erasing sector
Waiting for write to finish
Status: {writing: false} (0)
OK
Message: Erase sector addr: 0x70001000
Physical addr: 0x00001000
Erasing sector
Waiting for write to finish
Status: {writing: false} (0)
OK
Message: Dropped
Test: Erase done
Message: Init
PWR::CR3 = 0x00010044
RCC::AHB4ENR = 0x00000000
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
2026-05-15T12:00:24.166164Z  WARN probe_rs::probe::stlink: send_jtag_command 242 failed: SwdApFault
Error: Something during the interaction with the core went wrong
```
^ a problem is occuring outside of this codebase, cause currently unknown.

## Dependencies

Run the following requirements:

```bash
cargo install cargo-generate cargo-binutils target-gen
rustup component add llvm-tools-preview
```

## Developing the algorithm

Just run `cargo run`. It spits out the flash algo in the probe-rs YAML format and downloads it onto a target and makes a test run.
You will also be able to see RTT messages.

You can find the generated YAML in `target/definition.yaml`.

# License

This thingy is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Flash Algorithm for the MakerPnPControl-CORE board

This is a flash algorithm template for writing CMSIS-Pack flash algorithms in Rust.
It can be used to flash the external flash chip on the board using `probe-rs`.

## Current status

Incomplete, not usable yet due to issues:

```
Running `target-gen test template.yaml target/definition.yaml target\thumbv7em-none-eabihf\debug\makerpnpcore-w25q16jv`
Generating the YAML file in `"target/definition.yaml"`
Test: Erasing sectorwise and writing two pages ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e
0x00000020: 1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e
0x00000040: 3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e
0x00000060: 5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e
0x00000080: 7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e
0x000000a0: 9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6
Message: Erase sector addr: 0x70000000
Physical addr: 0x00000000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
..
Message: Status: {writing: true} (3)
..
OK
Message: Erase sector addr: 0x70001000
Physical addr: 0x00001000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
..
Message: Status: {writing: true} (3)
Status: {writing: true} (3)
...
OK
Message: Dropped
Test: Erase done
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000020: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000040: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000060: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000080: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x000000a0: ffffffffffffffffffffffffffffffffffffffffffffffff
Message: Blank check. logical_address: 0x70000000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00000000, size: 0x00001000 (4096)
read_quad_output. address: 0x00000000, data_length: 4096
Message: Blank check. logical_address: 0x70001000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00001000, size: 0x00001000 (4096)
read_quad_output. address: 0x00001000, data_length: 4096
Message: Dropped
Test: Writing two pages ...
Finished programming in 662.0024ms
Test: Write done
2026-05-15T20:17:14.584722Z  WARN probe_rs::probe::stlink: send_jtag_command 242 failed: SwdApFault
Error: An ARM specific error occurred.

Caused by:
    0: The debug probe encountered an error.
    1: An error which is specific to the debug probe in use occurred.
    2: Command failed with status SwdApFault.
```
^ a problem is occurring outside of this codebase, cause currently unknown.

the write is occuring, but likely the readback is failing, but it's unknown how the readback is occuring

previously the same error was occurring when the MCU attempted to read from the mapped flash address, since
the MCU is not in memory-mapped mode.

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

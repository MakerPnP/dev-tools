# Flash Algorithm for the MakerPnPControl-CORE board

This is a flash algorithm template for writing CMSIS-Pack flash algorithms in Rust.
It can be used to flash the external flash chip on the board using `probe-rs`.

## Current status

Working!

## Pre-requisites

```
cargo install cargo-generate cargo-binutils target-gen
rustup component add llvm-tools-preview
```

## Running

WARNING: This will erase the flash chip on the target board so be sure
you have the required information to restore the flash chip contents, if required.

1. Connect a supported debug probe to the MakerPnPControl-CORE board using the debug header.
2. Power on the debug probe and target board.
3. Run cargo
```
cargo run
```
4. This should run `target-gen` which will use the debug probe to connect to the board and
run a series of tests on the flash chip.
5. Afterwards you can find the generated YAML in `target/definition.yaml` and this can be used to
by probe-rs to write to the flash chip.

for example, you can now flash and verify a binary using commands like this:
```
 ../../../probe-rs/target/debug/probe-rs download --chip makerpnpcore-w25q16jv --chip-description-path ./target/definition.yaml test1.bin --binary-format=bin --base-address 0x70000000 --verify
      Erasing ✔ 100% [####################]   4.00 KiB @   5.88 KiB/s (took 1s)
  Programming ✔ 100% [####################]      256 B @   5.32 KiB/s (took 0s)
    Verifying ✔ 100% [####################]      256 B @   5.59 KiB/s (took 0s)
Finished in 0.89s
```

Note that you can not *read* from the flash chip using the probe-rs command line, as the `probe-rs read ...` command 
does NOT use the flash-api, it uses the core directly.  Changes to probe-rs would be required to support this.

### Gotchas

As part of this implementation a feedback issue was created on the probe-rs repo, if you're implementing a flash algo 
please read the feedback, it will save you from the hours of debugging headaches that was endured.

https://github.com/probe-rs/probe-rs/issues/4016

Four main gotchas:

1) interrupts are never called, due to lack of a vector table or vector table init code in probe-rs. thus you cannot
use dependencies like embassy-time and methods like 'block_for' or any non-blocking peripheral code that requires
interrupts to function.  

2) panic handlers are never called, for the same reason above, a vector table entry for a hard-fault handler is 
required for them to work.  you get a core lockup instead.

3) It was found that an earlier version of the impl works in target-gen but not probe-rs. with probe-rs it would crash
as soon as the program phase was started. it turned out there was a massive difference between how target-gen and
probe-rs uses the flash-algo-impl:

* probe-rs downloads the algo ONCE, and calls new/drop multiple times ON THE SAME INSTANCE.
* target-gen downloads the algo MULTIPLE-TIMES and calls new/drop ONCE on EACH INSTANCE.

This meant that the `new` code, which was calling `rtt_init_print` would panic when probe-rs called it for the second
time as `rtt_init_print` cannot be called more than once.

Other implementations seen on the internet probably had the same issue as `rtt_init_print` was commented out in them, 
who knows, just speculating... but for example see: https://github.com/kevswims/stm32h7-flash-example/blob/main/flash-algo/src/main.rs#L47-L50

The log from target-gen below also shows `LOADING ALGO` being printed multiple times, indicating that the global state is
gone between target-gen tests. Debug code was added to probe-rs to highlight this.

4) it's not currently possible to use the `read` function of probe-rs to read a flash chip, even if you've implemented
`read_flash`.  details in the feedback issue linked above.

### Log
```
Finished `test` profile [optimized + debuginfo] target(s) in 0.31s
Running `../../../probe-rs/target/debug/target-gen test template.yaml target/definition.yaml target\thumbv7em-none-eabihf\debug\makerpnpcore-w25q16jv`

Generating the YAML file in `"target/definition.yaml"`
Test: Erasing sectorwise and writing two pages ...
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Erase sector addr: 0x70000000
Physical addr: 0x00000000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
Message: Status: {writing: false} (0)
OK
Message: Erase sector addr: 0x70001000
Physical addr: 0x00001000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
Message: Status: {writing: false} (0)
OK
Message: Dropped
Test: Erase done
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Blank check. logical_address: 0x70000000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00000000, size: 0x00001000 (4096)
read_quad_output. address: 0x00000000, data_length: 4096
Message: Blank check. logical_address: 0x70001000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00001000, size: 0x00001000 (4096)
read_quad_output. address: 0x00001000, data_length: 4096
Message: Dropped
Test: Writing two pages ...
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Program Page. addr: 0x70000000, size: 0x00000100 (256)
page_program. address: 0x00000000, data_length: 256
Status: {writing: true} (3)
Status: {writing: true} (1)
Status: {writing: false} (0)
Message: Program Page. addr: 0x70000100, size: 0x00000100 (256)
page_program. address: 0x00000100, data_length: 256
Status: {writing: true} (3)
Status: {writing: true} (1)
Status: {writing: false} (0)
Message: Dropped
Finished programming in 670.3677ms
Test: Write done
Test: Reading back two pages (via API) ...
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Read. logical_address: 0x70000001
read_quad_output. address: 0x00000001, data_length: 256
Message: Dropped
Test: Write verified OK
Test: Erasing the entire chip and writing two pages ...
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Erase All
Status: {writing: true} (3)
Message: Status: {writing: false} (0)
Message: Dropped
Finished erasing in 6.1719137s
Test: Erase done
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Blank check. logical_address: 0x70000000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00000000, size: 0x00001000 (4096)
read_quad_output. address: 0x00000000, data_length: 4096
Message: Blank check. logical_address: 0x70001000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00001000, size: 0x00001000 (4096)
read_quad_output. address: 0x00001000, data_length: 4096
Message: Dropped
Test: Writing two pages ...
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Program Page. addr: 0x70000000, size: 0x00000100 (256)
page_program. address: 0x00000000, data_length: 256
Status: {writing: true} (3)
Status: {writing: true} (1)
Status: {writing: false} (0)
Message: Program Page. addr: 0x70000100, size: 0x00000100 (256)
page_program. address: 0x00000100, data_length: 256
Status: {writing: true} (3)
Status: {writing: true} (1)
Status: {writing: false} (0)
Message: Dropped
Finished programming in 668.9619ms
Test: Write done
Test: Reading back two pages (via API) ...
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Read. logical_address: 0x70000001
read_quad_output. address: 0x00000001, data_length: 256
Message: Dropped
Test: Write verified OK
Test: Erasing sectorwise and writing two pages double buffered ...
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Erase sector addr: 0x70000000
Physical addr: 0x00000000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
Message: Status: {writing: false} (0)
OK
Message: Erase sector addr: 0x70001000
Physical addr: 0x00001000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
Message: Status: {writing: true} (1)
Status: {writing: false} (0)
OK
Message: Dropped
Test: Erase done
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Blank check. logical_address: 0x70000000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00000000, size: 0x00001000 (4096)
read_quad_output. address: 0x00000000, data_length: 4096
Message: Blank check. logical_address: 0x70001000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00001000, size: 0x00001000 (4096)
read_quad_output. address: 0x00001000, data_length: 4096
Message: Dropped
Test: Writing two pages ...
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Program Page. addr: 0x70000000, size: 0x00000100 (256)
page_program. address: 0x00000000, data_length: 256
Status: {writing: true} (3)
Status: {writing: false} (0)
Message: Program Page. addr: 0x70000100, size: 0x00000100 (256)
page_program. address: 0x00000100, data_length: 256
Status: {writing: true} (3)
Status: {writing: true} (1)
Status: {writing: false} (0)
Message: Dropped
Finished programming in 671.5356ms
Test: Write done
Test: Verifying
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Verify. logical address: 1879048192, size: 256, verify_data: true 
Reading: 0x00000000, size: 0x00000100 (256)
read_quad_output. address: 0x00000000, data_length: 256
Message: Verify. logical address: 1879048448, size: 256, verify_data: true 
Reading: 0x00000100, size: 0x00000100 (256)
read_quad_output. address: 0x00000100, data_length: 256
Message: Dropped
Test: verification done
Test: Reading back two pages (via API) ...
********* LOADING ALGO ****************
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
Init complete, flash ID as-expected
Message: Read. logical_address: 0x70000001
read_quad_output. address: 0x00000001, data_length: 256
Message: Dropped
Test: Write verified OK
Finished in 14.9014131s

Process finished with exit code 0
```

# License

This thingy is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

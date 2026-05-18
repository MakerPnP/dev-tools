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

### Log
```
     Running `D:\Users\Hydra\Documents\dev\projects\makerpnp\dev-tools\makerpnpcore-w25q16jv\../../../probe-rs/target/debug/target-gen test template.yaml target/definition.yaml target\thumbv7em-none-eabihf\debug\makerpnpcore-w25q16jv`
Generating the YAML file in `"target/definition.yaml"`
Test: Erasing sectorwise and writing two pages ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff00010203040506 0708090a0b0c0d0e 0f10111213141516 1718191a1b1c1d1e
0x00000020: 1f20212223242526 2728292a2b2c2d2e 2f30313233343536 3738393a3b3c3d3e
0x00000040: 3f40414243444546 4748494a4b4c4d4e 4f50515253545556 5758595a5b5c5d5e
0x00000060: 5f60616263646566 6768696a6b6c6d6e 6f70717273747576 7778797a7b7c7d7e
0x00000080: 7f80818283848586 8788898a8b8c8d8e 8f90919293949596 9798999a9b9c
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
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000020: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000040: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000060: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000080: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffff
Message: Blank check. logical_address: 0x70000000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00000000, size: 0x00001000 (4096)
read_quad_output. address: 0x00000000, data_length: 4096
Message: Blank check. logical_address: 0x70001000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00001000, size: 0x00001000 (4096)
read_quad_output. address: 0x00001000, data_length: 4096
Message: Dropped
Test: Writing two pages ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000020: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000040: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000060: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000080: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffff
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
Finished programming in 194.4212ms
Test: Write done
Test: Reading back two pages (via API) ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff00010203040506 0708090a0b0c0d0e 0f10111213141516 1718191a1b1c1d1e
0x00000020: 1f20212223242526 2728292a2b2c2d2e 2f30313233343536 3738393a3b3c3d3e
0x00000040: 3f40414243444546 4748494a4b4c4d4e 4f50515253545556 5758595a5b5c5d5e
0x00000060: 5f60616263646566 6768696a6b6c6d6e 6f70717273747576 7778797a7b7c7d7e
0x00000080: 7f80818283848586 8788898a8b8c8d8e 8f90919293949596 9798999a9b9c
Message: Read. logical_address: 0x70000001
read_quad_output. address: 0x00000001, data_length: 256
Message: Dropped
Test: Write verified OK
Test: Erasing the entire chip and writing two pages ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff00010203040506 0708090a0b0c0d0e 0f10111213141516 1718191a1b1c1d1e
0x00000020: 1f20212223242526 2728292a2b2c2d2e 2f30313233343536 3738393a3b3c3d3e
0x00000040: 3f40414243444546 4748494a4b4c4d4e 4f50515253545556 5758595a5b5c5d5e
0x00000060: 5f60616263646566 6768696a6b6c6d6e 6f70717273747576 7778797a7b7c7d7e
0x00000080: 7f80818283848586 8788898a8b8c8d8e 8f90919293949596 9798999a9b9c
Message: Erase All
Status: {writing: true} (3)
Message: Status: {writing: false} (0)
Message: Dropped
Finished erasing in 5.6544135s
Test: Erase done
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000020: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000040: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000060: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000080: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffff
Message: Blank check. logical_address: 0x70000000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00000000, size: 0x00001000 (4096)
read_quad_output. address: 0x00000000, data_length: 4096
Message: Blank check. logical_address: 0x70001000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00001000, size: 0x00001000 (4096)
read_quad_output. address: 0x00001000, data_length: 4096
Message: Dropped
Test: Writing two pages ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000020: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000040: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000060: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000080: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffff
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
Finished programming in 193.1066ms
Test: Write done
Test: Reading back two pages (via API) ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff00010203040506 0708090a0b0c0d0e 0f10111213141516 1718191a1b1c1d1e
0x00000020: 1f20212223242526 2728292a2b2c2d2e 2f30313233343536 3738393a3b3c3d3e
0x00000040: 3f40414243444546 4748494a4b4c4d4e 4f50515253545556 5758595a5b5c5d5e
0x00000060: 5f60616263646566 6768696a6b6c6d6e 6f70717273747576 7778797a7b7c7d7e
0x00000080: 7f80818283848586 8788898a8b8c8d8e 8f90919293949596 9798999a9b9c
Message: Read. logical_address: 0x70000001
read_quad_output. address: 0x00000001, data_length: 256
Message: Dropped
Test: Write verified OK
Test: Erasing sectorwise and writing two pages double buffered ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff00010203040506 0708090a0b0c0d0e 0f10111213141516 1718191a1b1c1d1e
0x00000020: 1f20212223242526 2728292a2b2c2d2e 2f30313233343536 3738393a3b3c3d3e
0x00000040: 3f40414243444546 4748494a4b4c4d4e 4f50515253545556 5758595a5b5c5d5e
0x00000060: 5f60616263646566 6768696a6b6c6d6e 6f70717273747576 7778797a7b7c7d7e
0x00000080: 7f80818283848586 8788898a8b8c8d8e 8f90919293949596 9798999a9b9c
Message: Erase sector addr: 0x70000000
Physical addr: 0x00000000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
Message: Status: {writing: true} (1)
Status: {writing: false} (0)
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
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000020: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000040: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000060: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000080: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffff
Message: Blank check. logical_address: 0x70000000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00000000, size: 0x00001000 (4096)
read_quad_output. address: 0x00000000, data_length: 4096
Message: Blank check. logical_address: 0x70001000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00001000, size: 0x00001000 (4096)
read_quad_output. address: 0x00001000, data_length: 4096
Message: Dropped
Test: Writing two pages ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000020: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000040: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000060: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffffffff
0x00000080: ffffffffffffffff ffffffffffffffff ffffffffffffffff ffffffffffff
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
Finished programming in 194.1699ms
Test: Write done
Test: Verifying
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff00010203040506 0708090a0b0c0d0e 0f10111213141516 1718191a1b1c1d1e
0x00000020: 1f20212223242526 2728292a2b2c2d2e 2f30313233343536 3738393a3b3c3d3e
0x00000040: 3f40414243444546 4748494a4b4c4d4e 4f50515253545556 5758595a5b5c5d5e
0x00000060: 5f60616263646566 6768696a6b6c6d6e 6f70717273747576 7778797a7b7c7d7e
0x00000080: 7f80818283848586 8788898a8b8c8d8e 8f90919293949596 9798999a9b9c
Message: Verify. logical address: 1879048192, size: 256, verify_data: true 
Reading: 0x00000000, size: 0x00000100 (256)
read_quad_output. address: 0x00000000, data_length: 256
Message: Verify. logical address: 1879048448, size: 256, verify_data: true 
Reading: 0x00000100, size: 0x00000100 (256)
read_quad_output. address: 0x00000100, data_length: 256
Message: Dropped
Test: verification done
Test: Reading back two pages (via API) ...
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff00010203040506 0708090a0b0c0d0e 0f10111213141516 1718191a1b1c1d1e
0x00000020: 1f20212223242526 2728292a2b2c2d2e 2f30313233343536 3738393a3b3c3d3e
0x00000040: 3f40414243444546 4748494a4b4c4d4e 4f50515253545556 5758595a5b5c5d5e
0x00000060: 5f60616263646566 6768696a6b6c6d6e 6f70717273747576 7778797a7b7c7d7e
0x00000080: 7f80818283848586 8788898a8b8c8d8e 8f90919293949596 9798999a9b9c
Message: Read. logical_address: 0x70000001
read_quad_output. address: 0x00000001, data_length: 256
Message: Dropped
Test: Write verified OK
Finished in 8.7018932s
```

# License

This thingy is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

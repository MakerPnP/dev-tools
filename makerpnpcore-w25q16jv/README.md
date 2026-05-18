# Flash Algorithm for the MakerPnPControl-CORE board

This is a flash algorithm template for writing CMSIS-Pack flash algorithms in Rust.
It can be used to flash the external flash chip on the board using `probe-rs`.

## Current status

Issues have been identified in target-gen which, PR's will be created to fix them, the current output using an updated
`target-gen` is as below.

Incomplete, not usable yet due to the cpu core locking up, cause unknown

### Log

```
D:/Users/Hydra/.cargo/bin/cargo.exe run --color=always --package makerpnpcore-w25q16jv --bin makerpnpcore-w25q16jv --profile test
warning: field `fpga_creset_b` is never read
  --> src\main.rs:36:5
   |
33 | struct Algorithm {
   |        --------- field in this struct
...
36 |     fpga_creset_b: Output<'static>,
   |     ^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `makerpnpcore-w25q16jv` (bin "makerpnpcore-w25q16jv") generated 1 warning
    Finished `test` profile [optimized + debuginfo] target(s) in 0.28s
     Running `D:\Users\Hydra\Documents\dev\projects\makerpnp\dev-tools\makerpnpcore-w25q16jv\../../../probe-rs/target/debug/target-gen test --speed=1000 template.yaml target/definition.yaml target\thumbv7em-none-eabihf\debug\makerpnpcore-w25q16jv`
Generating the YAML file in `"target/definition.yaml"`
2026-05-18T13:59:40.289287Z DEBUG target_gen::parser: Trying to read 8 bytes from 0x20008680.
2026-05-18T13:59:40.289600Z DEBUG target_gen::parser: Segment address: 0x20000020
2026-05-18T13:59:40.289732Z DEBUG target_gen::parser: Segment size:    34240 bytes
2026-05-18T13:59:40.289877Z DEBUG target_gen::parser: Skipping segment.
2026-05-18T13:59:40.289993Z DEBUG target_gen::parser: Segment address: 0x200085e0
2026-05-18T13:59:40.290107Z DEBUG target_gen::parser: Segment size:    176 bytes
2026-05-18T13:59:40.290240Z DEBUG target_gen::parser: Trying to read 8 bytes from 0x20008688.
2026-05-18T13:59:40.290360Z DEBUG target_gen::parser: Segment address: 0x20000020
2026-05-18T13:59:40.290453Z DEBUG target_gen::parser: Segment size:    34240 bytes
2026-05-18T13:59:40.290572Z DEBUG target_gen::parser: Skipping segment.
2026-05-18T13:59:40.290682Z DEBUG target_gen::parser: Segment address: 0x200085e0
2026-05-18T13:59:40.290778Z DEBUG target_gen::parser: Segment size:    176 bytes
2026-05-18T13:59:40.290879Z DEBUG target_gen::parser: Trying to read 160 bytes from 0x200085e0.
2026-05-18T13:59:40.290957Z DEBUG target_gen::parser: Segment address: 0x20000020
2026-05-18T13:59:40.291027Z DEBUG target_gen::parser: Segment size:    34240 bytes
2026-05-18T13:59:40.291108Z DEBUG target_gen::parser: Skipping segment.
2026-05-18T13:59:40.291187Z DEBUG target_gen::parser: Segment address: 0x200085e0
2026-05-18T13:59:40.291261Z DEBUG target_gen::parser: Segment size:    176 bytes
2026-05-18T13:59:40.291376Z DEBUG target_gen::algorithm_binary: Program header: LOAD to VMA 0x20000020
2026-05-18T13:59:40.291509Z DEBUG target_gen::algorithm_binary: Program header: LOAD to VMA 0x200085e0
2026-05-18T13:59:40.292213Z DEBUG target_gen::parser: Found RTT control block at address 0x200080f8
2026-05-18T13:59:40.292402Z DEBUG target_gen::parser: Flash algorithm will be loaded at fixed address 0x20000020
2026-05-18T13:59:40.294119Z DEBUG target_gen::parser: Trying to read 8 bytes from 0x20008680.
2026-05-18T13:59:40.294248Z DEBUG target_gen::parser: Segment address: 0x20000020
2026-05-18T13:59:40.294327Z DEBUG target_gen::parser: Segment size:    34240 bytes
2026-05-18T13:59:40.294418Z DEBUG target_gen::parser: Skipping segment.
2026-05-18T13:59:40.294504Z DEBUG target_gen::parser: Segment address: 0x200085e0
2026-05-18T13:59:40.294637Z DEBUG target_gen::parser: Segment size:    176 bytes
2026-05-18T13:59:40.294724Z DEBUG target_gen::parser: Trying to read 8 bytes from 0x20008688.
2026-05-18T13:59:40.294809Z DEBUG target_gen::parser: Segment address: 0x20000020
2026-05-18T13:59:40.294881Z DEBUG target_gen::parser: Segment size:    34240 bytes
2026-05-18T13:59:40.294970Z DEBUG target_gen::parser: Skipping segment.
2026-05-18T13:59:40.295075Z DEBUG target_gen::parser: Segment address: 0x200085e0
2026-05-18T13:59:40.295157Z DEBUG target_gen::parser: Segment size:    176 bytes
2026-05-18T13:59:40.295243Z DEBUG target_gen::parser: Trying to read 160 bytes from 0x200085e0.
2026-05-18T13:59:40.295327Z DEBUG target_gen::parser: Segment address: 0x20000020
2026-05-18T13:59:40.295397Z DEBUG target_gen::parser: Segment size:    34240 bytes
2026-05-18T13:59:40.295473Z DEBUG target_gen::parser: Skipping segment.
2026-05-18T13:59:40.295552Z DEBUG target_gen::parser: Segment address: 0x200085e0
2026-05-18T13:59:40.295648Z DEBUG target_gen::parser: Segment size:    176 bytes
2026-05-18T13:59:40.295754Z DEBUG target_gen::algorithm_binary: Program header: LOAD to VMA 0x20000020
2026-05-18T13:59:40.295813Z DEBUG target_gen::algorithm_binary: Program header: LOAD to VMA 0x200085e0
2026-05-18T13:59:40.296273Z DEBUG target_gen::parser: Found RTT control block at address 0x200080f8
2026-05-18T13:59:40.296435Z DEBUG target_gen::parser: Flash algorithm will be loaded at fixed address 0x20000020
2026-05-18T13:59:40.558808Z DEBUG probe_rs::config::registry: Exact match for family name: makerpnpcore-w25q16jv
2026-05-18T13:59:40.837204Z DEBUG probe_rs::config::registry: Searching registry for chip with name makerpnpcore-w25q16jv
2026-05-18T13:59:40.837308Z DEBUG probe_rs::config::registry: Exact match for chip name: makerpnpcore-w25q16jv
2026-05-18T13:59:40.837417Z  INFO probe_rs::config::target: Using sequence Arm(DefaultArmSequence(()))
2026-05-18T13:59:40.837860Z  INFO attach: probe_rs::probe::stlink: Target voltage (VAPP): 3.30 V
2026-05-18T13:59:40.841534Z DEBUG valid_access_ports:valid_access_ports_allowlist: probe_rs::architecture::arm::ap::v1: AP V1(3) is not valid, IDR = 0 dp=Default dp=Default
2026-05-18T13:59:40.841671Z DEBUG debug_device_unlock: probe_rs::architecture::arm::sequences: debug_device_unlock - empty by default
2026-05-18T13:59:40.842049Z DEBUG debug_core_start: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } id=0
2026-05-18T13:59:40.842702Z DEBUG debug_core_start: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } id=0
2026-05-18T13:59:40.843979Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:40.844471Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:40.846260Z DEBUG probe_rs::architecture::arm::core::armv7m: The core is in locked up status as a result of an unrecoverable exception
2026-05-18T13:59:40.846354Z  INFO probe_rs::session: Halting core 0...
2026-05-18T13:59:40.849094Z  INFO probe_rs::session: Clearing breakpoints for core 0
2026-05-18T13:59:40.849382Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:40.849836Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:40.856545Z DEBUG probe_rs::session: Resuming core...
2026-05-18T13:59:40.856854Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:40.857289Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:40.859734Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Request), new reason is Request
2026-05-18T13:59:40.860890Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
Test: Erasing sectorwise and writing two pages ...
2026-05-18T13:59:40.863302Z DEBUG probe_rs::flashing::erase: Erasing 70000000..70002000
2026-05-18T13:59:40.863383Z DEBUG probe_rs::flashing::erase: Regions:
2026-05-18T13:59:40.863516Z DEBUG probe_rs::flashing::erase:     region: 0x70000000..0x70200000 (2097152 bytes)
2026-05-18T13:59:40.863608Z DEBUG probe_rs::flashing::loader: Available algorithms:
2026-05-18T13:59:40.863694Z DEBUG probe_rs::flashing::loader: Algorithm: makerpnpcore-w25q16jv for ["main"] @ 0x70000000 - 0x70200000  default? true
2026-05-18T13:59:40.863912Z DEBUG probe_rs::flashing::erase:      -- using algorithm: makerpnpcore-w25q16jv
2026-05-18T13:59:40.864031Z DEBUG probe_rs::flashing::erase: Erasing with algorithm: makerpnpcore-w25q16jv
2026-05-18T13:59:40.864137Z  INFO probe_rs::flashing::flash_algorithm: Chosen RAM to run the algo: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:40.864207Z  INFO probe_rs::flashing::flash_algorithm: Data will be loaded to: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:40.864628Z  INFO probe_rs::flashing::flash_algorithm: The flash algorithm will be configured with 8192 bytes of stack
2026-05-18T13:59:40.864724Z  INFO probe_rs::flashing::flash_algorithm: Stack top: 0x2000a7e8
2026-05-18T13:59:40.864804Z DEBUG probe_rs::flashing::flash_algorithm: Page buffers: [
    0x200085e8,
    0x200086e8,
]
2026-05-18T13:59:40.864967Z DEBUG probe_rs::flashing::flasher: Initializing the flash algorithm.
2026-05-18T13:59:40.865287Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:40.865768Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:40.866001Z DEBUG probe_rs::flashing::flasher: Reset and halt core 0
2026-05-18T13:59:40.868931Z DEBUG reset_and_halt: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 9, external: false, vcatch: true, dwttrap: false, bkpt: false, halted: true } timeout=500ms
2026-05-18T13:59:40.872129Z DEBUG probe_rs::flashing::flasher: Downloading algorithm code to 0x2000001c
2026-05-18T13:59:41.451140Z DEBUG probe_rs::flashing::flasher: RAM contents match flashing algo blob.
2026-05-18T13:59:41.451542Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:41.452002Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:41.452254Z DEBUG probe_rs::flashing::flasher: Preparing Flasher for operation Erase
2026-05-18T13:59:41.452317Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: Calling routine 0x20000549 (Some(1879048192), Some(0), Some(1), None), init=true)
2026-05-18T13:59:41.454566Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000548 should be: 0x0000000020000549
2026-05-18T13:59:41.456625Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000000 should be: 0x0000000070000000
2026-05-18T13:59:41.458671Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000000000 should be: 0x0000000000000000
2026-05-18T13:59:41.460931Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R2 0x2: 0x0000000000000001 should be: 0x0000000000000001
2026-05-18T13:59:41.463006Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R9 0x9: 0x00000000200085e0 should be: 0x00000000200085e0
2026-05-18T13:59:41.464974Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R13 0xd: 0x000000002000a7e8 should be: 0x000000002000a7e8
2026-05-18T13:59:41.466998Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:41.469119Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Multiple), new reason is Request
2026-05-18T13:59:41.470286Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:41.472293Z DEBUG Call to flash algorithm init: probe_rs::rtt: Initializing RTT (attempt 1)...
2026-05-18T13:59:41.472383Z DEBUG Call to flash algorithm init: probe_rs::rtt: Scanning at exact address: 0x200080f8
2026-05-18T13:59:41.474063Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() ptr = 0x2000639C
2026-05-18T13:59:41.475493Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() result = Some("Terminal")
2026-05-18T13:59:41.475968Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=2s
2026-05-18T13:59:41.505429Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000020: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000040: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000060: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000080: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
ffffffffff timeout=2s
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000020: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000040: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000060: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000080: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
ffffffffff
2026-05-18T13:59:41.506309Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=2s
2026-05-18T13:59:41.508155Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=2s
2026-05-18T13:59:41.510308Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=2s
2026-05-18T13:59:41.510406Z DEBUG probe_rs::flashing::erase:     sector: 0x70000000-0x70001000 (4096 bytes)
2026-05-18T13:59:41.510498Z  INFO probe_rs::flashing::flasher: Erasing sector at address 0x70000000
2026-05-18T13:59:41.510584Z DEBUG probe_rs::flashing::flasher: Calling routine 0x20000365 (Some(1879048192), None, None, None), init=false)
2026-05-18T13:59:41.512926Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000364 should be: 0x0000000020000365
2026-05-18T13:59:41.515184Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000000 should be: 0x0000000070000000
2026-05-18T13:59:41.517394Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:41.520862Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:41.521937Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:41.524377Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=25s
2026-05-18T13:59:41.547053Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Erase sector addr: 0x70000000
Physical addr: 0x00000000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
 timeout=25s
Message: Erase sector addr: 0x70000000
Physical addr: 0x00000000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
2026-05-18T13:59:41.569983Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=25s
2026-05-18T13:59:41.571839Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=25s
2026-05-18T13:59:41.573160Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Status: {writing: false} (0)
OK
 timeout=25s
Message: Status: {writing: false} (0)
OK
2026-05-18T13:59:41.574678Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=25s
2026-05-18T13:59:41.574748Z  INFO probe_rs::flashing::flasher: Done erasing sector. Result is 0. This took 64.1679ms
2026-05-18T13:59:41.574835Z DEBUG probe_rs::flashing::erase:     sector: 0x70001000-0x70002000 (4096 bytes)
2026-05-18T13:59:41.574890Z  INFO probe_rs::flashing::flasher: Erasing sector at address 0x70001000
2026-05-18T13:59:41.574980Z DEBUG probe_rs::flashing::flasher: Calling routine 0x20000365 (Some(1879052288), None, None, None), init=false)
2026-05-18T13:59:41.576995Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000364 should be: 0x0000000020000365
2026-05-18T13:59:41.579187Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070001000 should be: 0x0000000070001000
2026-05-18T13:59:41.581175Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:41.584360Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:41.585622Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:41.587811Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=25s
2026-05-18T13:59:41.609991Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Erase sector addr: 0x70001000
Physical addr: 0x00001000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
 timeout=25s
Message: Erase sector addr: 0x70001000
Physical addr: 0x00001000
Erasing sector
Waiting for write to finish
Status: {writing: true} (3)
2026-05-18T13:59:41.643574Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=25s
2026-05-18T13:59:41.645556Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=25s
2026-05-18T13:59:41.646949Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Status: {writing: false} (0)
OK
 timeout=25s
Message: Status: {writing: false} (0)
OK
2026-05-18T13:59:41.648464Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=25s
2026-05-18T13:59:41.648576Z  INFO probe_rs::flashing::flasher: Done erasing sector. Result is 0. This took 73.5989ms
2026-05-18T13:59:41.648642Z DEBUG probe_rs::flashing::flasher: Running uninit routine.
2026-05-18T13:59:41.648731Z DEBUG probe_rs::flashing::flasher: Calling routine 0x2000127d (Some(1), None, None, None), init=false)
2026-05-18T13:59:41.650716Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x000000002000127c should be: 0x000000002000127d
2026-05-18T13:59:41.652607Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000000000001 should be: 0x0000000000000001
2026-05-18T13:59:41.654630Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:41.658175Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:41.659255Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:41.661615Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=2s
2026-05-18T13:59:41.662345Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=2s
2026-05-18T13:59:41.664169Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=2s
2026-05-18T13:59:41.665323Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Dropped
 timeout=2s
Message: Dropped
2026-05-18T13:59:41.666811Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=2s
Test: Erase done
2026-05-18T13:59:41.666953Z DEBUG probe_rs::flashing::erase: Blank-checking 70000000..70002000
2026-05-18T13:59:41.667019Z DEBUG probe_rs::flashing::erase: Regions:
2026-05-18T13:59:41.667058Z DEBUG probe_rs::flashing::erase:     region: 0x70000000..0x70200000 (2097152 bytes)
2026-05-18T13:59:41.667116Z DEBUG probe_rs::flashing::loader: Available algorithms:
2026-05-18T13:59:41.667148Z DEBUG probe_rs::flashing::loader: Algorithm: makerpnpcore-w25q16jv for ["main"] @ 0x70000000 - 0x70200000  default? true
2026-05-18T13:59:41.667195Z DEBUG probe_rs::flashing::erase:      -- using algorithm: makerpnpcore-w25q16jv
2026-05-18T13:59:41.667247Z DEBUG probe_rs::flashing::erase: Blank-checking with algorithm: makerpnpcore-w25q16jv
2026-05-18T13:59:41.667322Z  INFO probe_rs::flashing::flash_algorithm: Chosen RAM to run the algo: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:41.667401Z  INFO probe_rs::flashing::flash_algorithm: Data will be loaded to: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:41.667757Z  INFO probe_rs::flashing::flash_algorithm: The flash algorithm will be configured with 8192 bytes of stack
2026-05-18T13:59:41.667835Z  INFO probe_rs::flashing::flash_algorithm: Stack top: 0x2000a7e8
2026-05-18T13:59:41.667872Z DEBUG probe_rs::flashing::flash_algorithm: Page buffers: [
    0x200085e8,
    0x200086e8,
]
2026-05-18T13:59:41.667948Z DEBUG probe_rs::flashing::flasher: Initializing the flash algorithm.
2026-05-18T13:59:41.668215Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:41.668682Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:41.668927Z DEBUG probe_rs::flashing::flasher: Reset and halt core 0
2026-05-18T13:59:41.674618Z DEBUG probe_rs::flashing::flasher: Downloading algorithm code to 0x2000001c
2026-05-18T13:59:42.253145Z DEBUG probe_rs::flashing::flasher: RAM contents match flashing algo blob.
2026-05-18T13:59:42.253464Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:42.253929Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:42.254150Z DEBUG probe_rs::flashing::flasher: Preparing Flasher for operation Verify
2026-05-18T13:59:42.254215Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: Calling routine 0x20000549 (Some(1879048192), Some(0), Some(3), None), init=true)
2026-05-18T13:59:42.256362Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000548 should be: 0x0000000020000549
2026-05-18T13:59:42.258382Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000000 should be: 0x0000000070000000
2026-05-18T13:59:42.260516Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000000000 should be: 0x0000000000000000
2026-05-18T13:59:42.262572Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R2 0x2: 0x0000000000000003 should be: 0x0000000000000003
2026-05-18T13:59:42.264676Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R9 0x9: 0x00000000200085e0 should be: 0x00000000200085e0
2026-05-18T13:59:42.266835Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R13 0xd: 0x000000002000a7e8 should be: 0x000000002000a7e8
2026-05-18T13:59:42.268901Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:42.270900Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Exception), new reason is Request
2026-05-18T13:59:42.272050Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:42.274008Z DEBUG Call to flash algorithm init: probe_rs::rtt: Initializing RTT (attempt 1)...
2026-05-18T13:59:42.274087Z DEBUG Call to flash algorithm init: probe_rs::rtt: Scanning at exact address: 0x200080f8
2026-05-18T13:59:42.275859Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() ptr = 0x2000639C
2026-05-18T13:59:42.277310Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() result = Some("Terminal")
2026-05-18T13:59:42.277734Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=2s
2026-05-18T13:59:42.306967Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000020: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000040: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000060: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000080: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
ffffffffff timeout=2s
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000020: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000040: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000060: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000080: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
ffffffffff
2026-05-18T13:59:42.307895Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=2s
2026-05-18T13:59:42.309751Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=2s
2026-05-18T13:59:42.311730Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=2s
2026-05-18T13:59:42.311864Z DEBUG probe_rs::flashing::erase:     sector: 0x70000000-0x70001000 (4096 bytes)
2026-05-18T13:59:42.311931Z  INFO probe_rs::flashing::flasher: Checking for blanked flash between address 0x70000000 and 0x70001000
2026-05-18T13:59:42.311982Z DEBUG probe_rs::flashing::flasher: Calling routine 0x20000021 (Some(1879048192), Some(4096), Some(255), None), init=false)
2026-05-18T13:59:42.314374Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000020 should be: 0x0000000020000021
2026-05-18T13:59:42.316373Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000000 should be: 0x0000000070000000
2026-05-18T13:59:42.318428Z DEBUG probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000001000 should be: 0x0000000000001000
2026-05-18T13:59:42.320537Z DEBUG probe_rs::flashing::flasher: content of R2 0x2: 0x00000000000000ff should be: 0x00000000000000ff
2026-05-18T13:59:42.322482Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:42.325782Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:42.326824Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:42.329314Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=10s
2026-05-18T13:59:42.330359Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=10s
2026-05-18T13:59:42.332157Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=10s
2026-05-18T13:59:42.335144Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Blank check. logical_address: 0x70000000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00000000, size: 0x00001000 (4096)
read_quad_output. address: 0x00000000, data_length: 4096
 timeout=10s
Message: Blank check. logical_address: 0x70000000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00000000, size: 0x00001000 (4096)
read_quad_output. address: 0x00000000, data_length: 4096
2026-05-18T13:59:42.336741Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=10s
2026-05-18T13:59:42.336803Z  INFO probe_rs::flashing::flasher: Done checking blank. Result is 0. This took 24.8217ms
2026-05-18T13:59:42.336877Z DEBUG probe_rs::flashing::erase:     sector: 0x70001000-0x70002000 (4096 bytes)
2026-05-18T13:59:42.336969Z  INFO probe_rs::flashing::flasher: Checking for blanked flash between address 0x70001000 and 0x70002000
2026-05-18T13:59:42.337059Z DEBUG probe_rs::flashing::flasher: Calling routine 0x20000021 (Some(1879052288), Some(4096), Some(255), None), init=false)
2026-05-18T13:59:42.339053Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000020 should be: 0x0000000020000021
2026-05-18T13:59:42.341003Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070001000 should be: 0x0000000070001000
2026-05-18T13:59:42.343394Z DEBUG probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000001000 should be: 0x0000000000001000
2026-05-18T13:59:42.345715Z DEBUG probe_rs::flashing::flasher: content of R2 0x2: 0x00000000000000ff should be: 0x00000000000000ff
2026-05-18T13:59:42.347769Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:42.350925Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:42.351932Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:42.354614Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=10s
2026-05-18T13:59:42.355358Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=10s
2026-05-18T13:59:42.357127Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=10s
2026-05-18T13:59:42.359848Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Blank check. logical_address: 0x70001000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00001000, size: 0x00001000 (4096)
read_quad_output. address: 0x00001000, data_length: 4096
 timeout=10s
Message: Blank check. logical_address: 0x70001000, size: 0x00001000 (4096), pattern: 0xff (0b11111111)
Reading: 0x00001000, size: 0x00001000 (4096)
read_quad_output. address: 0x00001000, data_length: 4096
2026-05-18T13:59:42.361500Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=10s
2026-05-18T13:59:42.361590Z  INFO probe_rs::flashing::flasher: Done checking blank. Result is 0. This took 24.5313ms
2026-05-18T13:59:42.361678Z DEBUG probe_rs::flashing::flasher: Running uninit routine.
2026-05-18T13:59:42.361756Z DEBUG probe_rs::flashing::flasher: Calling routine 0x2000127d (Some(3), None, None, None), init=false)
2026-05-18T13:59:42.364147Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x000000002000127c should be: 0x000000002000127d
2026-05-18T13:59:42.366119Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000000000003 should be: 0x0000000000000003
2026-05-18T13:59:42.368049Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:42.371202Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:42.372426Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:42.374901Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=2s
2026-05-18T13:59:42.375647Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=2s
2026-05-18T13:59:42.377509Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=2s
2026-05-18T13:59:42.378750Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Dropped
 timeout=2s
Message: Dropped
2026-05-18T13:59:42.380231Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=2s
Test: Writing two pages ...
2026-05-18T13:59:42.380374Z TRACE probe_rs::flashing::loader: Adding data at address 0x70000001 with size 256 bytes
2026-05-18T13:59:42.380444Z DEBUG probe_rs::flashing::loader: Committing FlashLoader!
2026-05-18T13:59:42.380497Z DEBUG probe_rs::flashing::loader: Contents of builder:
2026-05-18T13:59:42.380538Z DEBUG probe_rs::flashing::loader:     data: 0x70000001..0x70000101 (256 bytes)
2026-05-18T13:59:42.380593Z DEBUG probe_rs::flashing::loader: Flash algorithms:
2026-05-18T13:59:42.380669Z DEBUG probe_rs::flashing::loader:     algo makerpnpcore-w25q16jv: 0x70000000..0x70200000 (2097152 bytes)
2026-05-18T13:59:42.380740Z DEBUG probe_rs::flashing::loader: Regions:
2026-05-18T13:59:42.380794Z DEBUG probe_rs::flashing::loader:     region: 0x70000000..0x70200000 (2097152 bytes)
2026-05-18T13:59:42.380855Z DEBUG probe_rs::flashing::loader: Available algorithms:
2026-05-18T13:59:42.380916Z DEBUG probe_rs::flashing::loader: Algorithm: makerpnpcore-w25q16jv for ["main"] @ 0x70000000 - 0x70200000  default? true
2026-05-18T13:59:42.380958Z DEBUG probe_rs::flashing::loader:      -- using algorithm: makerpnpcore-w25q16jv
2026-05-18T13:59:42.381014Z  INFO probe_rs::flashing::flash_algorithm: Chosen RAM to run the algo: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:42.381076Z  INFO probe_rs::flashing::flash_algorithm: Data will be loaded to: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:42.381408Z  INFO probe_rs::flashing::flash_algorithm: The flash algorithm will be configured with 8192 bytes of stack
2026-05-18T13:59:42.381487Z  INFO probe_rs::flashing::flash_algorithm: Stack top: 0x2000a7e8
2026-05-18T13:59:42.381548Z DEBUG probe_rs::flashing::flash_algorithm: Page buffers: [
    0x200085e8,
    0x200086e8,
]
2026-05-18T13:59:42.384323Z DEBUG probe_rs::flashing::loader: Flashing ranges for algo: makerpnpcore-w25q16jv
2026-05-18T13:59:42.384399Z  INFO probe_rs::flashing::loader: Disabled double-buffering support for loader via passed option, though target supports it.
2026-05-18T13:59:42.384483Z DEBUG probe_rs::flashing::flasher: Starting program procedure.
2026-05-18T13:59:42.384539Z DEBUG probe_rs::flashing::flasher: Double Buffering enabled: false
2026-05-18T13:59:42.384579Z DEBUG probe_rs::flashing::flasher: Restoring unwritten bytes enabled: false
2026-05-18T13:59:42.384653Z DEBUG probe_rs::flashing::flasher: Initializing the flash algorithm.
2026-05-18T13:59:42.384933Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:42.385393Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:42.385708Z DEBUG probe_rs::flashing::flasher: Reset and halt core 0
2026-05-18T13:59:42.391627Z DEBUG probe_rs::flashing::flasher: Downloading algorithm code to 0x2000001c
2026-05-18T13:59:42.970200Z DEBUG probe_rs::flashing::flasher: RAM contents match flashing algo blob.
2026-05-18T13:59:42.970573Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:42.971083Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:42.971514Z DEBUG probe_rs::flashing::flasher: Preparing Flasher for operation Program
2026-05-18T13:59:42.971599Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: Calling routine 0x20000549 (Some(1879048192), Some(0), Some(2), None), init=true)
2026-05-18T13:59:42.973747Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000548 should be: 0x0000000020000549
2026-05-18T13:59:42.975751Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000000 should be: 0x0000000070000000
2026-05-18T13:59:42.977713Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000000000 should be: 0x0000000000000000
2026-05-18T13:59:42.979840Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R2 0x2: 0x0000000000000002 should be: 0x0000000000000002
2026-05-18T13:59:42.981804Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R9 0x9: 0x00000000200085e0 should be: 0x00000000200085e0
2026-05-18T13:59:42.983805Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R13 0xd: 0x000000002000a7e8 should be: 0x000000002000a7e8
2026-05-18T13:59:42.986029Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:42.988383Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Exception), new reason is Request
2026-05-18T13:59:42.989400Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:42.991397Z DEBUG Call to flash algorithm init: probe_rs::rtt: Initializing RTT (attempt 1)...
2026-05-18T13:59:42.991465Z DEBUG Call to flash algorithm init: probe_rs::rtt: Scanning at exact address: 0x200080f8
2026-05-18T13:59:42.993127Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() ptr = 0x2000639C
2026-05-18T13:59:42.994631Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() result = Some("Terminal")
2026-05-18T13:59:42.995111Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=2s
2026-05-18T13:59:43.024434Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000020: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000040: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000060: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000080: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
ffffffffff timeout=2s
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000020: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000040: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000060: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
0x00000080: ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
ffffffffff
2026-05-18T13:59:43.025500Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=2s
2026-05-18T13:59:43.027361Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=2s
2026-05-18T13:59:43.029180Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=2s
2026-05-18T13:59:43.029241Z DEBUG probe_rs::flashing::flasher:     programming region: 0x70000000..0x70200000 (2097152 bytes)
2026-05-18T13:59:43.029332Z  INFO probe_rs::flashing::flasher: Flashing page at address 0x70000000 with size: 256
2026-05-18T13:59:43.029405Z DEBUG probe_rs::flashing::flasher: Loading 256 bytes of data into RAM at address 0x200085e8

2026-05-18T13:59:43.031558Z  INFO probe_rs::flashing::flasher: Took 2.0982ms to download 256 byte page into ram
2026-05-18T13:59:43.031609Z DEBUG probe_rs::flashing::flasher: Calling routine 0x20000efd (Some(1879048192), Some(256), Some(536905192), None), init=false)
2026-05-18T13:59:43.033708Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000efc should be: 0x0000000020000efd
2026-05-18T13:59:43.035693Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000000 should be: 0x0000000070000000
2026-05-18T13:59:43.037704Z DEBUG probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000000100 should be: 0x0000000000000100
2026-05-18T13:59:43.039590Z DEBUG probe_rs::flashing::flasher: content of R2 0x2: 0x00000000200085e8 should be: 0x00000000200085e8
2026-05-18T13:59:43.041572Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:43.044879Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:43.045996Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:43.048429Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=3s
2026-05-18T13:59:43.049508Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=3s
2026-05-18T13:59:43.051304Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=3s
2026-05-18T13:59:43.054072Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Program Page. addr: 0x70000000, size: 0x00000100 (256)
page_program. address: 0x00000000, data_length: 256
Status: {writing: true} (3)
Status: {writing: false} (0)
 timeout=3s
Message: Program Page. addr: 0x70000000, size: 0x00000100 (256)
page_program. address: 0x00000000, data_length: 256
Status: {writing: true} (3)
Status: {writing: false} (0)
2026-05-18T13:59:43.055704Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=3s
2026-05-18T13:59:43.055776Z  INFO probe_rs::flashing::flasher: Flashing took: 26.4697ms
2026-05-18T13:59:43.055849Z  INFO probe_rs::flashing::flasher: Flashing page at address 0x70000100 with size: 256
2026-05-18T13:59:43.055931Z DEBUG probe_rs::flashing::flasher: Loading 256 bytes of data into RAM at address 0x200085e8

2026-05-18T13:59:43.058074Z  INFO probe_rs::flashing::flasher: Took 2.0824ms to download 256 byte page into ram
2026-05-18T13:59:43.058173Z DEBUG probe_rs::flashing::flasher: Calling routine 0x20000efd (Some(1879048448), Some(256), Some(536905192), None), init=false)
2026-05-18T13:59:43.060283Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000efc should be: 0x0000000020000efd
2026-05-18T13:59:43.062432Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000100 should be: 0x0000000070000100
2026-05-18T13:59:43.064759Z DEBUG probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000000100 should be: 0x0000000000000100
2026-05-18T13:59:43.066895Z DEBUG probe_rs::flashing::flasher: content of R2 0x2: 0x00000000200085e8 should be: 0x00000000200085e8
2026-05-18T13:59:43.068832Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:43.072375Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:43.073380Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:43.075945Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=3s
2026-05-18T13:59:43.076724Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=3s
2026-05-18T13:59:43.078555Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=3s
2026-05-18T13:59:43.081058Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Program Page. addr: 0x70000100, size: 0x00000100 (256)
page_program. address: 0x00000100, data_length: 256
Status: {writing: true} (3)
Status: {writing: false} (0)
 timeout=3s
Message: Program Page. addr: 0x70000100, size: 0x00000100 (256)
page_program. address: 0x00000100, data_length: 256
Status: {writing: true} (3)
Status: {writing: false} (0)
2026-05-18T13:59:43.082605Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=3s
2026-05-18T13:59:43.082677Z  INFO probe_rs::flashing::flasher: Flashing took: 26.8291ms
2026-05-18T13:59:43.082737Z DEBUG probe_rs::flashing::flasher: Running uninit routine.
2026-05-18T13:59:43.082849Z DEBUG probe_rs::flashing::flasher: Calling routine 0x2000127d (Some(2), None, None, None), init=false)
2026-05-18T13:59:43.084900Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x000000002000127c should be: 0x000000002000127d
2026-05-18T13:59:43.087062Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000000000002 should be: 0x0000000000000002
2026-05-18T13:59:43.089002Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:43.092478Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:43.093507Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:43.095988Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=2s
2026-05-18T13:59:43.096730Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=2s
2026-05-18T13:59:43.098559Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=2s
2026-05-18T13:59:43.099769Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Dropped
 timeout=2s
Message: Dropped
2026-05-18T13:59:43.101337Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=2s
Finished programming in 716.7543ms
2026-05-18T13:59:43.101498Z DEBUG probe_rs::flashing::loader: Committing RAM!
Test: Write done
Test: Reading back two pages (via API) ...
2026-05-18T13:59:43.101739Z  INFO probe_rs::flashing::flash_algorithm: Chosen RAM to run the algo: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:43.101812Z  INFO probe_rs::flashing::flash_algorithm: Data will be loaded to: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:43.102169Z  INFO probe_rs::flashing::flash_algorithm: The flash algorithm will be configured with 8192 bytes of stack
2026-05-18T13:59:43.102236Z  INFO probe_rs::flashing::flash_algorithm: Stack top: 0x2000a7e8
2026-05-18T13:59:43.102296Z DEBUG probe_rs::flashing::flash_algorithm: Page buffers: [
    0x200085e8,
    0x200086e8,
]
2026-05-18T13:59:43.102371Z DEBUG probe_rs::flashing::flasher: Initializing the flash algorithm.
2026-05-18T13:59:43.102622Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:43.103092Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:43.103323Z DEBUG probe_rs::flashing::flasher: Reset and halt core 0
2026-05-18T13:59:43.109111Z DEBUG probe_rs::flashing::flasher: Downloading algorithm code to 0x2000001c
2026-05-18T13:59:43.687662Z DEBUG probe_rs::flashing::flasher: RAM contents match flashing algo blob.
2026-05-18T13:59:43.688071Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:43.688511Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:43.688775Z DEBUG probe_rs::flashing::flasher: Preparing Flasher for operation Verify
2026-05-18T13:59:43.688842Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: Calling routine 0x20000549 (Some(1879048192), Some(0), Some(3), None), init=true)
2026-05-18T13:59:43.690907Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000548 should be: 0x0000000020000549
2026-05-18T13:59:43.692897Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000000 should be: 0x0000000070000000
2026-05-18T13:59:43.694910Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000000000 should be: 0x0000000000000000
2026-05-18T13:59:43.696915Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R2 0x2: 0x0000000000000003 should be: 0x0000000000000003
2026-05-18T13:59:43.698932Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R9 0x9: 0x00000000200085e0 should be: 0x00000000200085e0
2026-05-18T13:59:43.700941Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R13 0xd: 0x000000002000a7e8 should be: 0x000000002000a7e8
2026-05-18T13:59:43.702883Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:43.705163Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Exception), new reason is Request
2026-05-18T13:59:43.706195Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:43.708154Z DEBUG Call to flash algorithm init: probe_rs::rtt: Initializing RTT (attempt 1)...
2026-05-18T13:59:43.708222Z DEBUG Call to flash algorithm init: probe_rs::rtt: Scanning at exact address: 0x200080f8
2026-05-18T13:59:43.709899Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() ptr = 0x2000639C
2026-05-18T13:59:43.711328Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() result = Some("Terminal")
2026-05-18T13:59:43.711768Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=2s
2026-05-18T13:59:43.741024Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e
0x00000020: 1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e
0x00000040: 3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e
0x00000060: 5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e
0x00000080: 7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e
9fa0a1a2a3 timeout=2s
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e
0x00000020: 1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e
0x00000040: 3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e
0x00000060: 5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e
0x00000080: 7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e
9fa0a1a2a3
2026-05-18T13:59:43.742131Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=2s
2026-05-18T13:59:43.743942Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=2s
2026-05-18T13:59:43.745868Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=2s
2026-05-18T13:59:43.745955Z DEBUG probe_rs::flashing::flasher: Calling routine 0x200011a9 (Some(1879048193), Some(256), Some(536905192), None), init=false)
2026-05-18T13:59:43.748118Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x00000000200011a8 should be: 0x00000000200011a9
2026-05-18T13:59:43.750140Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000001 should be: 0x0000000070000001
2026-05-18T13:59:43.752448Z DEBUG probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000000100 should be: 0x0000000000000100
2026-05-18T13:59:43.754578Z DEBUG probe_rs::flashing::flasher: content of R2 0x2: 0x00000000200085e8 should be: 0x00000000200085e8
2026-05-18T13:59:43.756587Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:43.759688Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:43.760659Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:43.762892Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=30s
2026-05-18T13:59:43.763605Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=30s
2026-05-18T13:59:43.765372Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=30s
2026-05-18T13:59:43.767500Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Read. logical_address: 0x70000001
read_quad_output. address: 0x00000001, data_length: 256
 timeout=30s
Message: Read. logical_address: 0x70000001
read_quad_output. address: 0x00000001, data_length: 256
2026-05-18T13:59:43.769000Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=30s
2026-05-18T13:59:43.771415Z DEBUG probe_rs::flashing::flasher: Running uninit routine.
2026-05-18T13:59:43.771480Z DEBUG probe_rs::flashing::flasher: Calling routine 0x2000127d (Some(3), None, None, None), init=false)
2026-05-18T13:59:43.773683Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x000000002000127c should be: 0x000000002000127d
2026-05-18T13:59:43.775705Z DEBUG probe_rs::flashing::flasher: content of R0 0x0: 0x0000000000000003 should be: 0x0000000000000003
2026-05-18T13:59:43.777618Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:43.781050Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:43.782064Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:43.784301Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=2s
2026-05-18T13:59:43.785115Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=2s
2026-05-18T13:59:43.786927Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=2s
2026-05-18T13:59:43.788294Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Dropped
 timeout=2s
Message: Dropped
2026-05-18T13:59:43.789821Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=2s
Test: Write verified OK
Test: Erasing the entire chip and writing two pages ...
2026-05-18T13:59:43.790094Z DEBUG probe_rs::flashing::erase: Erasing all...
2026-05-18T13:59:43.790171Z DEBUG probe_rs::flashing::erase: Regions:
2026-05-18T13:59:43.790254Z DEBUG probe_rs::flashing::erase:     region: 0x70000000..0x70200000 (2097152 bytes)
2026-05-18T13:59:43.790357Z DEBUG probe_rs::flashing::loader: Available algorithms:
2026-05-18T13:59:43.790439Z DEBUG probe_rs::flashing::loader: Algorithm: makerpnpcore-w25q16jv for ["main"] @ 0x70000000 - 0x70200000  default? true
2026-05-18T13:59:43.790564Z DEBUG probe_rs::flashing::erase:      -- using algorithm: makerpnpcore-w25q16jv
2026-05-18T13:59:43.790644Z  INFO probe_rs::flashing::flash_algorithm: Chosen RAM to run the algo: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:43.790727Z  INFO probe_rs::flashing::flash_algorithm: Data will be loaded to: RamRegion { name: Some("DTCM"), range: 20000000..20020000, cores: ["main"], is_alias: false, access: Some(MemoryAccess { read: true, write: true, execute: true, boot: true }) }
2026-05-18T13:59:43.791202Z  INFO probe_rs::flashing::flash_algorithm: The flash algorithm will be configured with 8192 bytes of stack
2026-05-18T13:59:43.791305Z  INFO probe_rs::flashing::flash_algorithm: Stack top: 0x2000a7e8
2026-05-18T13:59:43.791366Z DEBUG probe_rs::flashing::flash_algorithm: Page buffers: [
    0x200085e8,
    0x200086e8,
]
2026-05-18T13:59:43.791477Z DEBUG probe_rs::flashing::erase: Erasing with algorithm: makerpnpcore-w25q16jv
2026-05-18T13:59:43.791557Z DEBUG probe_rs::flashing::erase:      -- chip erase supported, doing it.
2026-05-18T13:59:43.791627Z DEBUG probe_rs::flashing::flasher: Initializing the flash algorithm.
2026-05-18T13:59:43.791882Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:43.792316Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:43.792522Z DEBUG probe_rs::flashing::flasher: Reset and halt core 0
2026-05-18T13:59:43.798043Z DEBUG probe_rs::flashing::flasher: Downloading algorithm code to 0x2000001c
2026-05-18T13:59:44.376886Z DEBUG probe_rs::flashing::flasher: RAM contents match flashing algo blob.
2026-05-18T13:59:44.377218Z DEBUG attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:44.377633Z DEBUG attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:44.377856Z DEBUG probe_rs::flashing::flasher: Preparing Flasher for operation Erase
2026-05-18T13:59:44.377941Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: Calling routine 0x20000549 (Some(1879048192), Some(0), Some(1), None), init=true)
2026-05-18T13:59:44.379983Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R15 0xf: 0x0000000020000548 should be: 0x0000000020000549
2026-05-18T13:59:44.381940Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R0 0x0: 0x0000000070000000 should be: 0x0000000070000000
2026-05-18T13:59:44.383899Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R1 0x1: 0x0000000000000000 should be: 0x0000000000000000
2026-05-18T13:59:44.385895Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R2 0x2: 0x0000000000000001 should be: 0x0000000000000001
2026-05-18T13:59:44.387879Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R9 0x9: 0x00000000200085e0 should be: 0x00000000200085e0
2026-05-18T13:59:44.389853Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R13 0xd: 0x000000002000a7e8 should be: 0x000000002000a7e8
2026-05-18T13:59:44.392020Z DEBUG Call to flash algorithm init: probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:44.393911Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Exception), new reason is Request
2026-05-18T13:59:44.394853Z DEBUG Call to flash algorithm init:run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:44.396844Z DEBUG Call to flash algorithm init: probe_rs::rtt: Initializing RTT (attempt 1)...
2026-05-18T13:59:44.396892Z DEBUG Call to flash algorithm init: probe_rs::rtt: Scanning at exact address: 0x200080f8
2026-05-18T13:59:44.398638Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() ptr = 0x2000639C
2026-05-18T13:59:44.400142Z TRACE Call to flash algorithm init: probe_rs::rtt::channel: read_c_string() result = Some("Terminal")
2026-05-18T13:59:44.400549Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=2s
2026-05-18T13:59:44.429835Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e
0x00000020: 1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e
0x00000040: 3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e
0x00000060: 5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e
0x00000080: 7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e
9fa0a1a2a3 timeout=2s
Message: Init
PWR::CR3 = 0x00010044 Cr3 { bypass: false, ldoen: false, sden: true, sdexthp: false, sdlevel: Reset, vbe: false, vbrs: false, sdextrdy: true, usb33den: false, usbregen: false, usb33rdy: false }
RCC::AHB4ENR = 0x00000000 Ahb4enr { gpioaen: false, gpioben: false, gpiocen: false, gpioden: false, gpioeen: false, gpiofen: false, gpiogen: false, gpiohen: false, gpioien: false, gpiojen: false, gpioken: false, crcen: false, bdmaen: false, adc3en: false, hsemen: false, bkpsramen: false }
WWDG1::cr = 0x0000007f Cr { t: 127, wdga: false }
Full init
FlashID: [239, 64, 21]
read_quad_output. address: 0x00000000, data_length: 256
0x00000000: ff000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e
0x00000020: 1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e
0x00000040: 3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e
0x00000060: 5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e
0x00000080: 7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e
9fa0a1a2a3
2026-05-18T13:59:44.430732Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core: DFSR: Dfsr { .0: 3, external: false, vcatch: false, dwttrap: false, bkpt: true, halted: true } timeout=2s
2026-05-18T13:59:44.432583Z DEBUG Call to flash algorithm init:wait_for_completion:status: probe_rs::architecture::arm::core::cortex_m: Semihosting check pc=0x2000001c instruction=0xbe0x0 timeout=2s
2026-05-18T13:59:44.434477Z DEBUG Call to flash algorithm init:wait_for_completion: probe_rs::flashing::flasher: Routine returned 0. timeout=2s
2026-05-18T13:59:44.434604Z DEBUG probe_rs::flashing::flasher: Erasing entire chip.
2026-05-18T13:59:44.434686Z DEBUG probe_rs::flashing::flasher: Calling routine 0x2000023f (None, None, None, None), init=false)
2026-05-18T13:59:44.436688Z DEBUG probe_rs::flashing::flasher: content of R15 0xf: 0x000000002000023e should be: 0x000000002000023f
2026-05-18T13:59:44.438878Z DEBUG probe_rs::flashing::flasher: content of R14 0xe: 0x000000002000001d should be: 0x000000002000001d
2026-05-18T13:59:44.442252Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Breakpoint(Unknown)), new reason is Request
2026-05-18T13:59:44.443318Z DEBUG run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:44.445609Z DEBUG wait_for_completion: probe_rs::flashing::flasher: Waiting for routine call completion. timeout=40s
2026-05-18T13:59:44.467414Z DEBUG wait_for_completion: probe_rs::flashing::flasher: RTT(Terminal): Erase All
Status: {writing: true} (3)
 timeout=40s
Message: Erase All
Status: {writing: true} (3)
2026-05-18T13:59:45.397586Z DEBUG wait_for_completion:status: probe_rs::architecture::arm::core::armv7m: The core is in locked up status as a result of an unrecoverable exception timeout=40s
2026-05-18T13:59:45.397710Z DEBUG probe_rs::flashing::flasher: Routine call failed: Err(UnexpectedCoreStatus { status: LockedUp })
Failed erasing in 1.6061719s
2026-05-18T13:59:45.398124Z DEBUG session_drop:attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:45.398530Z DEBUG session_drop:attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:45.399111Z DEBUG session_drop: probe_rs::architecture::arm::core::armv7m: The core is in locked up status as a result of an unrecoverable exception
2026-05-18T13:59:45.399165Z  INFO session_drop: probe_rs::session: Halting core 0...
2026-05-18T13:59:45.401399Z  INFO session_drop: probe_rs::session: Clearing breakpoints for core 0
2026-05-18T13:59:45.401656Z DEBUG session_drop:attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:45.402067Z DEBUG session_drop:attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:45.408066Z DEBUG session_drop: probe_rs::session: Resuming core...
2026-05-18T13:59:45.408326Z DEBUG session_drop:attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:45.408812Z DEBUG session_drop:attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
2026-05-18T13:59:45.410947Z DEBUG session_drop:run: probe_rs::architecture::arm::core::armv7m: Reason for halt has changed, old reason was Halted(Request), new reason is Request
2026-05-18T13:59:45.411978Z DEBUG session_drop:run: probe_rs::architecture::arm::core::armv7m: Cached halt reason: Halted(Request)
2026-05-18T13:59:45.414123Z DEBUG session_drop:attach_to_core: probe_rs::architecture::arm::ap::memory_ap: reading IDR: IDR { REVISION: 8, DESIGNER: JEP106Code({ cc: 0x04, id: 0x3b } => Some("ARM Ltd")), CLASS: MemAp, _RES0: 0, VARIANT: 0, TYPE: AmbaAhb3 } core_index=0
2026-05-18T13:59:45.414534Z DEBUG session_drop:attach_to_core: probe_rs::architecture::arm::ap: Writing AP register CSW, value=CSW { DbgSwEnable: true, HNONSEC: false, MasterType: true, Allocate: false, Cacheable: true, Bufferable: false, Privileged: true, Data: true, SPIDEN: true, TrInProg: false, DeviceEn: true, AddrInc: Single, Size: U32, _reserved_bits: 0 } core_index=0
Error: Failed to erase the whole chip.

Caused by:
    The core entered an unexpected status: LockedUp.

Stack backtrace:
   0: std::backtrace_rs::backtrace::win64::trace
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\std\src\..\..\backtrace\src\backtrace\win64.rs:85
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2: std::backtrace::Backtrace::create
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\std\src\backtrace.rs:331
   3: std::backtrace::Backtrace::capture
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\std\src\backtrace.rs:296
   4: anyhow::error::impl$1::from<enum2$<probe_rs::flashing::error::FlashError> >
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\anyhow-1.0.102\src\backtrace.rs:10
   5: core::result::impl$28::from_residual<tuple$<>,enum2$<probe_rs::flashing::error::FlashError>,anyhow::Error>
             at D:\Users\Hydra\.rustup\toolchains\1.94.0-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\result.rs:2189
   6: target_gen::commands::test::run_flash_erase
             at D:\Users\Hydra\Documents\dev\projects\probe-rs\target-gen\src\commands\test.rs:346
   7: target_gen::commands::test::cmd_test
             at D:\Users\Hydra\Documents\dev\projects\probe-rs\target-gen\src\commands\test.rs:207
   8: target_gen::main::async_block$0
             at D:\Users\Hydra\Documents\dev\projects\probe-rs\target-gen\src\main.rs:179
   9: tokio::runtime::park::impl$4::block_on::closure$0<enum2$<target_gen::main::async_block_env$0> >
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\runtime\park.rs:284
  10: tokio::task::coop::with_budget
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\task\coop\mod.rs:167
  11: tokio::task::coop::budget
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\task\coop\mod.rs:133
  12: tokio::runtime::park::CachedParkThread::block_on<enum2$<target_gen::main::async_block_env$0> >
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\runtime\park.rs:284
  13: tokio::runtime::context::blocking::BlockingRegionGuard::block_on<enum2$<target_gen::main::async_block_env$0> >
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\runtime\context\blocking.rs:66
  14: tokio::runtime::scheduler::multi_thread::impl$0::block_on::closure$0<enum2$<target_gen::main::async_block_env$0> >
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\runtime\scheduler\multi_thread\mod.rs:92
  15: tokio::runtime::context::runtime::enter_runtime<tokio::runtime::scheduler::multi_thread::impl$0::block_on::closure_env$0<enum2$<target_gen::main::async_block_env$0> >,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\runtime\context\runtime.rs:65
  16: tokio::runtime::scheduler::multi_thread::MultiThread::block_on<enum2$<target_gen::main::async_block_env$0> >
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\runtime\scheduler\multi_thread\mod.rs:91
  17: tokio::runtime::runtime::Runtime::block_on_inner<enum2$<target_gen::main::async_block_env$0> >
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\runtime\runtime.rs:373
  18: tokio::runtime::runtime::Runtime::block_on<enum2$<target_gen::main::async_block_env$0> >
             at D:\Users\Hydra\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.52.3\src\runtime\runtime.rs:345
  19: target_gen::main
             at D:\Users\Hydra\Documents\dev\projects\probe-rs\target-gen\src\main.rs:212
  20: core::ops::function::FnOnce::call_once<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(),tuple$<> >
             at D:\Users\Hydra\.rustup\toolchains\1.94.0-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ops\function.rs:250
  21: core::hint::black_box
             at D:\Users\Hydra\.rustup\toolchains\1.94.0-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\hint.rs:482
  22: std::sys::backtrace::__rust_begin_short_backtrace<enum2$<core::result::Result<tuple$<>,anyhow::Error> > (*)(),enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
             at D:\Users\Hydra\.rustup\toolchains\1.94.0-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\backtrace.rs:166
  23: std::rt::lang_start::closure$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
             at D:\Users\Hydra\.rustup\toolchains\1.94.0-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\rt.rs:206
  24: std::rt::lang_start_internal::closure$0
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\std\src\rt.rs:175
  25: std::panicking::catch_unwind::do_call
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\std\src\panicking.rs:581
  26: std::panicking::catch_unwind
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\std\src\panicking.rs:544
  27: std::panic::catch_unwind
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\std\src\panic.rs:359
  28: std::rt::lang_start_internal
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\std\src\rt.rs:171
  29: std::rt::lang_start<enum2$<core::result::Result<tuple$<>,anyhow::Error> > >
             at D:\Users\Hydra\.rustup\toolchains\1.94.0-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\rt.rs:205
  30: main
  31: invoke_main
             at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  32: __scrt_common_main_seh
             at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  33: BaseThreadInitThunk
  34: RtlUserThreadStart
error: process didn't exit successfully: `D:\Users\Hydra\Documents\dev\projects\makerpnp\dev-tools\makerpnpcore-w25q16jv\../../../probe-rs/target/debug/target-gen test --speed=1000 template.yaml target/definition.yaml target\thumbv7em-none-eabihf\debug\makerpnpcore-w25q16jv` (exit code: 1)

Process finished with exit code 1
```

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

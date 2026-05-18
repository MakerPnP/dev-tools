//! Implementation of the flash-algorithm for the MakerPnPControl-CORE board
//! PCB Revision: RevA1
//!
//! Run with this environment setup to see more log output
//! ```
//! RUST_LOG=trace,nusb=info,probe_rs::probe=info
//! ```
#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::{Debug, Formatter, Write};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::ospi::{AddressSize, DummyCycles, Ospi, OspiError, OspiWidth, TransferConfig};
use embassy_stm32::{ospi, Peri};
use embassy_stm32::ospi::{ChipSelectHighTime, FIFOThresholdLevel, MemorySize, MemoryType, WrapSize};
use embassy_stm32::peripherals::{PC3, PD4, PE10, PF4, PG12, PH3};
use flash_algorithm::*;
use rtt_target::{rprint, rprintln, rtt_init_print};

#[repr(u32)]
enum ErrorCodes {
    UnknownError = 1,
    BusError = 2,
    VerificationError = 3,
}

const OCTOSPI2_MEMORY_MAPPED_ADDRESS: u32 = 0x70000000;
const FLASH_SIZE: u32 = 0x200000;
const ERASED_VALUE: u8 = 0xff;
const PAGE_SIZE: u16 = 0x100;

struct Algorithm {
    flash: Flash,
    led: Output<'static>,
    fpga_creset_b: Output<'static>,
}

algorithm!(Algorithm, {
    device_name: "W25Q16JVUXIQ",
    device_type: DeviceType::ExtSpi,
    flash_address: OCTOSPI2_MEMORY_MAPPED_ADDRESS,
    flash_size: FLASH_SIZE,
    page_size: PAGE_SIZE,
    empty_value: ERASED_VALUE,
    program_time_out: 3_000, // tPP_MAX = 3ms, tPP_TYP = 0.4ms
    erase_time_out: 25_000, // tCE_MAX = 25s, tCE_TYP = 5s
    sectors: [{
        size: 0x1000,
        address: 0x0,
    }]
});

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        rtt_init_print!();
        rprintln!("Init");

        // Initialize peripherals & RCC.
        let p = rcc_setup::stm32h735g_init();

        // Output pin PA8 (also MCO)
        let led = Output::new(p.PA8, Level::High, Speed::Low);

        // on the test board, there is an FPGA connected to the OCTOSPI data lines so that the FPGA
        // can boot from the flash, with an unprogrammed FPGA it will have weak pull-ups on every IO pin
        // and these need to be disabled before the flash can be communicated with.
        // or, if the FPGA is running, it needs to be stopped first so that it doesn't interfere with
        // flash operations.

        // hold FPGA in RESET mode
        let mut fpga_creset_b = Output::new(p.PF15, Level::Low, Speed::Low);
        fpga_creset_b.set_low();

        let resources = FlashResources {
            ospi: p.OCTOSPI2,
            clk: p.PF4,  // P2_CLK
            d0: p.PD4,  // P1_IO4
            d1: p.PH3,  // P1_IO5
            d2: p.PC3,  // P1_IO6
            d3: p.PE10, // P1_IO7
            ncs: p.PG12, // P2_NCS
        };

        // TODO: Add setup code for the flash algorithm.
        let mut flash = Flash::new(resources);
        let reset_period = flash.reset()
            .map_err(|_ospi_error|ErrorCode::new(ErrorCodes::UnknownError as u32).unwrap())?;

        block_for(reset_period);


        let flash_id = flash.read_id()
            .map_err(|_ospi_error|ErrorCode::new(ErrorCodes::UnknownError as u32).unwrap())?;
        rprintln!("FlashID: {:?}", flash_id);
        if flash_id != [0xef, 0x40, 0x15] {
            return Err(ErrorCode::new(flash_id[0] as u32).unwrap());
        }

        let mut buffer: [u8; PAGE_SIZE as usize] = [0; PAGE_SIZE as usize];

        if let Ok(buffer) = flash.read_quad_output_blocking(0x00000000, &mut buffer) {
            dump_chunk(&buffer);
        }

        rprintln!("Init complete, flash ID as-expected");
        Ok(Self { flash, led, fpga_creset_b })
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        rprintln!("Erase All");
        self.flash.chip_erase_blocking()
            .map_err(|_ospi_error|ErrorCode::new(ErrorCodes::UnknownError as u32).unwrap())?;
        Ok(())
    }

    fn erase_sector(&mut self, address: u32) -> Result<(), ErrorCode> {
        // TODO validate the logical address is within range the memory mapped range
        // Err(ErrorCode::new(0x....).unwrap())
        rprintln!("Erase sector addr: 0x{:08x}", address);

        let physical_address = Self::logical_to_physical(address);
        rprintln!("Physical addr: 0x{:08x}", physical_address);

        // TODO validate that physical address starts on a sector boundary (0 <= n < size - sector_size)?
        //      or does probe-rs already enforce these?
        // Err(ErrorCode::new(0x....).unwrap())

        self.flash.erase_sector_blocking(physical_address)
            .map_err(|_ospi_error|ErrorCode::new(ErrorCodes::BusError as u32).unwrap())?;
        Ok(())
    }

    fn program_page(&mut self, address: u32, buffer: &[u8]) -> Result<(), ErrorCode> {
        let len = buffer.len();
        rprintln!("Program Page. addr: 0x{:08x}, size: 0x{:08x} ({})", address, len, len);
        let physical_address = Self::logical_to_physical(address);
        self.flash.program_page_blocking(physical_address, buffer)
            .map_err(|_ospi_error|ErrorCode::new(ErrorCodes::BusError as u32).unwrap())?;
        Ok(())
    }

    fn verify(
        &mut self,
        _address: u32,
        _size: u32,
        _data: Option<&[u8]>,
    ) -> Result<(), u32> {
        rprintln!("Verify. logical address: {}, size: {}",_address, _size);
        //todo!()
        // just return the supplied first address as the error address
        Err(_address)
        //Ok(())
    }

    fn read_flash(&mut self, address: u32, data: &mut [u8]) -> Result<(), ErrorCode> {
        rprintln!("Read. logical_address: 0x{:08x}", address);
        let physical_address = Self::logical_to_physical(address);
        self.flash.read_quad_output_blocking(physical_address, data)
            .map_err(|_ospi_error|ErrorCode::new(ErrorCodes::BusError as u32).unwrap())?;

        Ok(())
    }

    fn blank_check(&mut self, address: u32, size: u32, pattern: u8) -> Result<(), ErrorCode> {
        rprintln!("Blank check. logical_address: 0x{:08x}, size: 0x{:08x} ({}), pattern: 0x{:02x} (0b{:08b})", address, size, size, pattern, pattern);
        let physical_address = Self::logical_to_physical(address);

        let mut chunk_buffer: [u8; 4096] = [0; 4096];

        let mut remaining: usize = size as usize;
        while remaining > 0 {
            let chunk_size: usize = remaining.min(4096);
            let chunk = &mut chunk_buffer[0..chunk_size];
            rprintln!("Reading: 0x{:08x}, size: 0x{:08x} ({})", physical_address, chunk.len(), chunk.len());

            self.flash.read_quad_output_blocking(physical_address, chunk)
                .map_err(|_ospi_error|ErrorCode::new(ErrorCodes::BusError as u32).unwrap())?;

            if false {
                dump_chunk(chunk);
            }
            remaining -= chunk_size;

            for (_index, byte) in chunk.iter().enumerate() {
                if *byte != pattern {
                    // TODO is would be good to report the offset and the actual byte value that was different.
                    return Err(ErrorCode::new(ErrorCodes::VerificationError as u32).unwrap())
                }
            }
        }

        Ok(())
    }
}

fn dump_chunk(chunk: &[u8]) {
    const ITEMS_PER_LINE: usize = 32;

    // time allowed for the per-line rtt buffer to be read.
    let rtt_line_period = 3 * 1000;

    let lines = (chunk.len() + ITEMS_PER_LINE - 1) / ITEMS_PER_LINE;

    let mut offset: usize = 0;
    for _ in 0..lines {
        rprint!("0x{:08x}: ", offset);
        for index in 0..ITEMS_PER_LINE {
            rprint!("{:02x}", chunk[offset]);
            if index % 8 == 0 {
                rprint!("")
            }
            offset += 1;
        }

        rprintln!("");

        block_for(rtt_line_period);
    }
}

impl Algorithm {
    fn logical_to_physical(address: u32) -> u32 {
        address - OCTOSPI2_MEMORY_MAPPED_ADDRESS
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        // TODO: Add code here to uninitialize the flash algorithm.
        rprintln!("Dropped");
        self.led.set_low();
    }
}

mod commands {
    pub const CMD_READ_ID: u8 = 0x9F;
    pub const CMD_ENABLE_RESET: u8 = 0x66;
    pub const CMD_RESET_DEVICE: u8 = 0x99;
    pub const CMD_CHIP_ERASE: u8 = 0xC7;
    pub const CMD_WRITE_ENABLE: u8 = 0x06;
    pub const CMD_READ_STATUS_1: u8 = 0x05;
    pub const CMD_SECTOR_ERASE: u8 = 0x20;
    pub const CMD_FAST_READ_QUAD_OUTPUT: u8 = 0x6b;

    pub const CMD_PAGE_PROGRAM: u8 = 0x02;
}
use commands::*;

struct FlashResources {
    ospi: Peri<'static, embassy_stm32::peripherals::OCTOSPI2>,
    clk: Peri<'static, PF4>,
    d0: Peri<'static, PD4>,
    d1: Peri<'static, PH3>,
    d2: Peri<'static, PC3>,
    d3: Peri<'static, PE10>,
    ncs: Peri<'static, PG12>,
}

enum FlashError {
    #[allow(unused)]
    OspiError(OspiError)
}

struct Flash {
    ospi: Ospi<'static, embassy_stm32::peripherals::OCTOSPI2, embassy_stm32::mode::Blocking>,
}

impl Flash {
    fn new(r: FlashResources) -> Self {
        let ospi_config = ospi::Config {
            fifo_threshold: FIFOThresholdLevel::_16Bytes,
            memory_type: MemoryType::Standard,
            device_size: MemorySize::_2MiB,
            chip_select_high_time: ChipSelectHighTime::_1Cycle,
            free_running_clock: false,
            clock_mode: false,
            wrap_size: WrapSize::None,

            // ?
            clock_prescaler: 4,
            // ?
            sample_shifting: false,
            delay_hold_quarter_cycle: false,
            chip_select_boundary: 0,
            // ?
            delay_block_bypass: true,
            max_transfer: 0,
            refresh: 0,
        };

        let ospi = embassy_stm32::ospi::Ospi::new_blocking_quadspi(
            r.ospi,
            r.clk,  // P2_CLK
            r.d0,  // P1_IO4
            r.d1,  // P1_IO5
            r.d2,  // P1_IO6
            r.d3, // P1_IO7
            r.ncs, // P2_NCS
            ospi_config,
        );

        Self { ospi }
    }

    /// reset the flash chip, the caller must wait for the returned time before issuing further commannds
    pub fn reset(&mut self) -> Result<u32, FlashError> {
        self.exec_command(CMD_ENABLE_RESET)?;
        self.exec_command(CMD_RESET_DEVICE)?;

        Ok(30)
    }

    pub fn exec_command(&mut self, cmd: u8) -> Result<(), FlashError> {
        let transaction: TransferConfig = TransferConfig {
            iwidth: OspiWidth::SING,
            isize: AddressSize::_8Bit,
            adwidth: OspiWidth::NONE,
            dwidth: OspiWidth::NONE,
            instruction: Some(cmd as u32),
            ..Default::default()
        };
        self.ospi.blocking_command(&transaction).map_err(FlashError::OspiError)
    }

    pub fn read_id(&mut self) -> Result<[u8; 3], FlashError> {
        let mut buffer = [0; 3];
        let transaction: TransferConfig = TransferConfig {
            iwidth: OspiWidth::SING,
            isize: AddressSize::_8Bit,
            adwidth: OspiWidth::NONE,
            dwidth: OspiWidth::SING,
            instruction: Some(CMD_READ_ID as u32),
            ..Default::default()
        };
        self.ospi.blocking_read(&mut buffer, transaction).map_err(FlashError::OspiError)?;
        Ok(buffer)
    }

    pub fn chip_erase_blocking(&mut self) -> Result<(), FlashError> {
        self.exec_command(CMD_WRITE_ENABLE)?;
        self.exec_command(CMD_CHIP_ERASE)?;
        self.wait_write_finish(250 * 1000)
    }

    pub fn read_status1(&mut self) -> Result<StatusReg, FlashError> {
        let mut buffer = [0_u8];
        let transaction: TransferConfig = TransferConfig {
            iwidth: OspiWidth::SING,
            isize: AddressSize::_8Bit,
            adwidth: OspiWidth::NONE,
            dwidth: OspiWidth::SING,
            instruction: Some(CMD_READ_STATUS_1 as u32),
            ..Default::default()
        };
        self.ospi.blocking_read(&mut buffer, transaction).map(|_| {
            let status = StatusReg::from_raw(buffer[0]);

            status
        })
            .map_err(FlashError::OspiError)
    }

    fn wait_write_finish(&mut self, re_check_duration: u32) -> Result<(), FlashError> {
        let mut recent_status: Option<StatusReg> = None;

        loop {
            let status = self.read_status1()?;
            let print = match &recent_status {
                Some(previous) if *previous != status => true,
                None => true,
                _ => false,
            };
            if print {
                rprintln!("Status: {:?} ({})", status, status.value());
            }
            if !status.is_writing() {
                break
            }

            recent_status.replace(status);

            block_for(re_check_duration);
        }

        Ok(())
    }

    fn erase_sector_blocking(&mut self, address: u32) -> Result<(), FlashError> {
        self.exec_command(CMD_WRITE_ENABLE)?;

        let transaction: TransferConfig = TransferConfig {
            iwidth: OspiWidth::SING,
            isize: AddressSize::_8Bit,
            adwidth: OspiWidth::SING,
            adsize: AddressSize::_24bit,
            address: Some(address),
            instruction: Some(CMD_SECTOR_ERASE as u32),
            ..Default::default()
        };
        rprintln!("Erasing sector");
        self.ospi.blocking_command(&transaction)
            .map_err(FlashError::OspiError)?;
        rprintln!("Waiting for write to finish");
        self.wait_write_finish(5 * 1000)?;
        rprintln!("OK");
        Ok(())
    }

    pub fn read_quad_output_blocking<'b>(&mut self, address: u32, buffer: &'b mut [u8]) -> Result<&'b mut [u8], FlashError>{
        let transaction: TransferConfig = TransferConfig {
            iwidth: OspiWidth::SING,
            instruction: Some(CMD_FAST_READ_QUAD_OUTPUT as u32),
            isize: AddressSize::_8Bit,

            adwidth: OspiWidth::SING,
            address: Some(address),
            adsize: AddressSize::_24bit,

            dwidth: OspiWidth::QUAD,

            dummy: DummyCycles::_8,
            ..Default::default()
        };
        rprintln!("read_quad_output. address: 0x{:08x}, data_length: {:?}", address, buffer.len());
        self.ospi.blocking_read(buffer, transaction)
            .map_err(FlashError::OspiError)?;
        Ok(buffer)
    }

    pub(crate) fn program_page_blocking(&mut self, address: u32, buffer: &[u8]) -> Result<(), FlashError> {
        // TODO check that buffer length <= page size

        self.exec_command(CMD_WRITE_ENABLE)?;

        let transaction: TransferConfig = TransferConfig {
            iwidth: OspiWidth::SING,
            instruction: Some(CMD_PAGE_PROGRAM as u32),
            isize: AddressSize::_8Bit,

            adwidth: OspiWidth::SING,
            address: Some(address),
            adsize: AddressSize::_24bit,

            dwidth: OspiWidth::SING,

            ..Default::default()
        };
        rprintln!("page_program. address: 0x{:08x}, data_length: {:?}", address, buffer.len());

        self.ospi.blocking_write(buffer, transaction).unwrap();

        self.wait_write_finish(100)?;
        Ok(())
    }
}

mod status_register_bits {
    pub const STATUS_WRITE_IN_PROGRESS: u8 = 1 << 0;
    pub const STATUS_WRITE_ENABLED: u8 = 1 << 1;
}

pub use status_register_bits::*;

#[derive(Eq, PartialEq)]
struct StatusReg(u8);

impl StatusReg {

    pub fn from_raw(value: u8) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn is_writing(&self) -> bool {
        self.0 & STATUS_WRITE_IN_PROGRESS == 1
    }
}

impl Debug for StatusReg {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_char('{')?;
        f.write_fmt(format_args!("writing: {:?}", self.is_writing()))?;
        f.write_char('}')
    }
}

mod rcc_setup {

    use embassy_stm32::rcc::mux::Fmcsel;
    use embassy_stm32::rcc::{Hse, HseMode, *};
    use embassy_stm32::time::Hertz;
    use embassy_stm32::{pac, Config, Peripherals};
    use rtt_target::rprintln;

    /// Sets up clocks for the stm32h735g mcu
    /// change this if you plan to use a different microcontroller
    pub fn stm32h735g_init() -> Peripherals {
        // setup power and clocks for an stm32h735g-dk run from an external 25 Mhz external oscillator
        let mut config = Config::default();
        config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(50),
            mode: HseMode::Oscillator,
        });
        config.rcc.hsi = None;
        config.rcc.csi = false;
        config.rcc.pll1 = Some(Pll {
            source: PllSource::Hse,
            prediv: PllPreDiv::Div4,  // 12.5Mhz
            mul: PllMul::Mul44,       // 550Mhz
            divp: Some(PllDiv::Div1), // 550Mhz
            divq: Some(PllDiv::Div4), // 110Mhz
            divr: Some(PllDiv::Div2), // 275Mhz
        });
        config.rcc.pll2 = Some(Pll {
            source: PllSource::Hse,
            prediv: PllPreDiv::Div5,  // 10Mhz
            mul: PllMul::Mul40,       // 400Mhz
            divp: Some(PllDiv::Div5), // 80Mhz
            divq: Some(PllDiv::Div2), // 200Mhz
            divr: Some(PllDiv::Div3), // 133.33Mhz (for OSPI)
        });
        config.rcc.pll3 = Some(Pll {
            source: PllSource::Hse,
            prediv: PllPreDiv::Div25, // 2Mhz
            mul: PllMul::Mul96,       // 192Mhz
            divp: Some(PllDiv::Div1), // 192Mhz
            divq: Some(PllDiv::Div8), // 24Mhz
            divr: Some(PllDiv::Div4), // 48Mhz
        });
        config.rcc.voltage_scale = VoltageScale::Scale0;
        config.rcc.supply_config = SupplyConfig::DirectSMPS;
        config.rcc.sys = Sysclk::Pll1P; // 550Mhz
        config.rcc.d1c_pre = AHBPrescaler::Div1; // 550Mhz
        config.rcc.ahb_pre = AHBPrescaler::Div2; // 275Mhz
        config.rcc.apb1_pre = APBPrescaler::Div2; // 137.5Mhz
        config.rcc.apb2_pre = APBPrescaler::Div2; // 137.5Mhz
        config.rcc.apb3_pre = APBPrescaler::Div2; // 137.5Mhz
        config.rcc.apb4_pre = APBPrescaler::Div2; // 137.5Mhz

        config.rcc.mux.octospisel = Fmcsel::Pll2R; // 133.33Mhz

        let reg = pac::PWR.cr3().read();
        rprintln!("PWR::CR3 = 0x{:08x} {:?}", reg.0, reg);
        let reg = pac::RCC.ahb4enr().read();
        rprintln!("RCC::AHB4ENR = 0x{:08x} {:?}", reg.0, reg);
        let reg = pac::WWDG1.cr().read();
        rprintln!("WWDG1::cr = 0x{:08x} {:?}", reg.0, reg);

        rprintln!("Full init");
        embassy_stm32::init(config)
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    rprintln!("I panicked");

    unsafe {
        core::arch::asm!("udf #0");
        core::hint::unreachable_unchecked();
    }
}

fn block_for(micros: u32) {

    // naive implementation for testing
    for _ in 0..micros {
        unsafe {
            asm!("nop");
        }
    }

}
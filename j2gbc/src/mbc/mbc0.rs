use log::error;

use super::Mbc;
use crate::error::ExecutionError;
use crate::mem::{Address, ExtendedAddress, MemDevice, Ram, RNG_EXT_RAM, RNG_ROM_BANK1};

pub struct Mbc0 {
    rom: Vec<u8>,
    ram: Ram,
}

impl Mbc0 {
    pub fn new(rom: Vec<u8>) -> Mbc0 {
        Mbc0 {
            rom,
            ram: Ram::new(RNG_EXT_RAM.len()),
        }
    }
}

impl MemDevice for Mbc0 {
    fn read(&self, a: Address) -> Result<u8, ExecutionError> {
        if a.in_(RNG_ROM_BANK1) {
            Ok(self.rom[a.0 as usize])
        } else if a.in_(RNG_EXT_RAM) {
            self.ram.read(a - RNG_EXT_RAM.0)
        } else {
            error!("Address out of range for MBC 0");
            Err(ExecutionError::BusError)
        }
    }

    fn write(&mut self, a: Address, v: u8) -> Result<(), ExecutionError> {
        if a.in_(RNG_EXT_RAM) {
            self.ram.write(a - RNG_EXT_RAM.0, v)
        } else {
            error!("Unknown MBC0 register {}", a);
            Err(ExecutionError::BusError)
        }
    }
}

impl Mbc for Mbc0 {
    fn map_address_into_rom(&self, a: Address) -> ExtendedAddress {
        ExtendedAddress(u32::from(a.0))
    }

    fn get_sram(&self) -> &[u8] {
        self.ram.data.as_slice()
    }

    fn set_sram(&mut self, buf: &[u8]) {
        self.ram.data[..buf.len()].clone_from_slice(buf);
    }
}

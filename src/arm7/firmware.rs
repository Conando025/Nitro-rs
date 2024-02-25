use crate::{arm7::enter_critical_section, aux::Flags};
use core::{
    mem::{size_of, MaybeUninit},
    ptr::{read_volatile, write_volatile},
};

use super::leave_critical_section;

/**
 * # unsafe
 *
 * relies on REG_SPI_CNT to be set correctly
 */
unsafe fn spi_write_address(addr: usize) {
    write_spi(((addr >> 16) & 0xFF) as u8);
    write_spi(((addr >> 08) & 0xFF) as u8);
    write_spi(((addr >> 00) & 0xFF) as u8);
}

#[allow(unreachable_code)]
pub unsafe fn read_firmware<T: Sized>(addr: usize, destination: *mut T) {
    let old_ime = enter_critical_section();

    let read_command = SPI_ENABLE | SPI_BYTE_MODE | SPI_CONTINUOUS | SPI_DEVICE_FIRMWARE;
    write_volatile(REG_SPI_CNT, read_command);

    write_spi(FIRMWARE_READ);

    spi_write_address(addr);

    let buffer = destination as *mut u8;

    let size = size_of::<T>();

    for i in 0..size {
        buffer.add(i).write(read_spi());
    }

    write_volatile(REG_SPI_CNT, SPI_DISABLE);

    leave_critical_section(old_ime);
}

type Page = [u8; 256];

pub unsafe fn write_firmware<T: Sized>(addr: usize, source: &T) -> Result<(), FirmwareWriteError> {
    use FirmwareWriteError::*;

    let mut size = size_of::<T>();

    if addr & 0xff != 0 || size & 0xff != 0 {
        return Err(Allignment);
    }

    let base: *const Page = (source as *const T).cast();

    while size > 0 {
        size -= 256;
        let next_page: &Page = &*base.byte_add(size);
        let Ok(_) = write_firmware_page(addr + size, next_page) else {
            return Err(PageWrite);
        };
    }

    Ok(())
}

unsafe fn write_firmware_page(addr: usize, buffer: &Page) -> Result<(), ()> {
    let mut page_buffer: MaybeUninit<Page> = MaybeUninit::uninit();

    read_firmware(addr, page_buffer.as_mut_ptr());

    let page_buffer = page_buffer.assume_init();

    if compare_pages(&page_buffer, buffer) {
        return Ok(());
    }

    let old_ime = enter_critical_section();

    let read_command = SPI_ENABLE | SPI_CONTINUOUS | SPI_DEVICE_NVRAM;

    write_volatile(REG_SPI_CNT, read_command);
    write_spi(FIRMWARE_WREN);
    write_volatile(REG_SPI_CNT, SPI_DISABLE);
    //TODO: Not sure if this is needed but its what the refrence do so further testing needed
    write_volatile(REG_SPI_CNT, read_command);
    write_spi(FIRMWARE_RDSR);
    //Write Enable Latch
    while !read_spi().check(SPI_WRITE_ENABLED) {}
    write_volatile(REG_SPI_CNT, SPI_DISABLE);

    write_volatile(REG_SPI_CNT, read_command);
    write_spi(FIRMWARE_PW);
    spi_write_address(addr);

    for &byte in buffer {
        write_spi(byte);
    }

    write_volatile(REG_SPI_CNT, SPI_DISABLE);

    //wait for programming to finish
    write_volatile(REG_SPI_CNT, read_command);
    write_spi(FIRMWARE_RDSR);
    while read_spi().check(SPI_WORKING) {}
    write_volatile(REG_SPI_CNT, SPI_DISABLE);

    leave_critical_section(old_ime);

    let mut page_buffer: MaybeUninit<Page> = MaybeUninit::uninit();
    read_firmware(addr, page_buffer.as_mut_ptr());
    let page_buffer = page_buffer.assume_init();

    if compare_pages(&page_buffer, buffer) {
        Ok(())
    } else {
        Err(())
    }
}

fn compare_pages(a: &Page, b: &Page) -> bool {
    for (&x, &y) in a.iter().zip(b.iter()) {
        if x != y {
            return false;
        }
    }
    true
}

pub enum FirmwareWriteError {
    Allignment,
    PageWrite,
}

const FIRMWARE_READ: u8 = 0x03;
const FIRMWARE_WREN: u8 = 0x06;
const FIRMWARE_RDSR: u8 = 0x05;
const FIRMWARE_PW: u8 = 0x0A;

const REG_SPI_DATA: *mut u16 = 0x040001C2 as *mut u16;
const REG_SPI_CNT: *mut u16 = 0x040001C0 as *mut u16;

//SPI_CNT Flags
const SPI_DISABLE: u16 = 0;
const SPI_ENABLE: u16 = 1 << 15;
const SPI_BYTE_MODE: u16 = 1 << 10;
const SPI_CONTINUOUS: u16 = 1 << 11;
const SPI_BUSY: u16 = 1 << 7;
//TODO: Why are there two names for the same flag???
const SPI_DEVICE_FIRMWARE: u16 = 1 << 8;
const SPI_DEVICE_NVRAM: u16 = 1 << 8;

//SPI Result flag data
const SPI_WORKING: u8 = 1 << 0;
const SPI_WRITE_ENABLED: u8 = 1 << 1;

#[inline(always)]
unsafe fn read_spi() -> u8 {
    read_write_spi(0)
}

#[inline(always)]
unsafe fn write_spi(data: u8) {
    read_write_spi(data);
}

unsafe fn read_write_spi(data: u8) -> u8 {
    write_volatile(REG_SPI_DATA, data as u16);
    while read_volatile(REG_SPI_CNT).check(SPI_BUSY) {}
    read_volatile(REG_SPI_DATA) as u8
}

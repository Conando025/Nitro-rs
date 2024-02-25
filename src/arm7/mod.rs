#![allow(unused)]

use core::{
    mem::{self, size_of, transmute, MaybeUninit},
    ptr::{self, read_volatile, write_volatile},
};

#[repr(C)]
struct PersonalData {}

pub fn initalize_user_settings() {
    unsafe {
        let mut slot0_data: MaybeUninit<PersonalData> = MaybeUninit::uninit();
        let mut slot0_count: MaybeUninit<u8> = MaybeUninit::uninit();
        let mut slot0_crc: MaybeUninit<u8> = MaybeUninit::uninit();

        let mut slot1_data: MaybeUninit<PersonalData> = MaybeUninit::uninit();
        let mut slot1_count: MaybeUninit<u8> = MaybeUninit::uninit();
        let mut slot1_crc: MaybeUninit<u8> = MaybeUninit::uninit();

        let mut user_settings_baseaddr: MaybeUninit<usize> = MaybeUninit::uninit();
        read_firmware(0x20, user_settings_baseaddr.as_mut_ptr());
        let user_settings_baseaddr = user_settings_baseaddr.assume_init();

        let slot0_addr = user_settings_baseaddr * 8;
        let slot1_addr = user_settings_baseaddr * 8 + 0x100;

        read_firmware(slot0_addr + 0x00, slot0_data.as_mut_ptr());
        read_firmware(slot0_addr + 0x70, slot0_count.as_mut_ptr());
        read_firmware(slot0_addr + 0x72, slot0_crc.as_mut_ptr());

        read_firmware(slot1_addr + 0x00, slot1_data.as_mut_ptr());
        read_firmware(slot1_addr + 0x70, slot1_count.as_mut_ptr());
        read_firmware(slot1_addr + 0x72, slot1_crc.as_mut_ptr());

        let slot0_data = slot0_data.assume_init();
        let slot1_data = slot1_data.assume_init();

        let slot0_calculated_crc = swiCRC16(0xffff, &slot0_data);
        let slot1_calculated_crc = swiCRC16(0xffff, &slot1_data);

        let slot0_valid = slot0_calculated_crc == slot0_crc.assume_init();
        let slot1_valid = slot1_calculated_crc == slot1_crc.assume_init();

        let mut current_settings_slot = &slot0_data;

        if !(slot0_valid || slot1_valid) {
            return;
        } else if slot0_valid && slot1_valid {
            let slot0_count = slot0_count.assume_init();
            let slot1_count = slot1_count.assume_init();
            current_settings_slot = if slot1_count == (slot0_count + 1) & 0x7f {
                &slot1_data
            } else {
                &slot0_data
            };
        } else if slot1_valid {
            current_settings_slot = &slot1_data;
        }
        write_firmware(0x2FFFC80, current_settings_slot);
    }
}

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
unsafe fn read_firmware<T: Sized>(addr: usize, destination: *mut T){
    todo!("Enter Critical Section");

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

    todo!("Exit Critical Section");
}

type Page = [u8; 256];

unsafe fn write_firmware<T: Sized>(addr: usize, source: &T) -> Result<(), FirmwareWriteError> {
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

    todo!("Enter Critical Section");

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
    while read_spi().check(SPI_WORKING) {};
    write_volatile(REG_SPI_CNT, SPI_DISABLE);

    todo!("Exit Critical Section");

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
    for (&x, &y) in a.into_iter().zip(b.into_iter()) {
        if x != y {
            return false;
        }
    }
    true
}

enum FirmwareWriteError {
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

fn swiCRC16<T>(crc: u16, data_addr: &T) -> u8 {
    todo!()
}

trait Flags {
    fn check(self, other: Self) -> bool;
}

impl Flags for u8 {
    fn check(self, other: Self) -> bool {
        self & other != 0
    }
}

impl Flags for u16 {
    fn check(self, other: Self) -> bool {
        self & other != 0
    }
}
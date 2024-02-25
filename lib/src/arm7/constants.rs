// Firmware commands
pub const FIRMWARE_PP: u8 =   0x02;
pub const FIRMWARE_READ: u8 = 0x03;
pub const FIRMWARE_WRDI: u8 = 0x04;
pub const FIRMWARE_RDSR: u8 = 0x05;
pub const FIRMWARE_WREN: u8 = 0x06;
pub const FIRMWARE_PW: u8 =   0x0A;
pub const FIRMWARE_FAST: u8 = 0x0B;
pub const FIRMWARE_RDID: u8 = 0x9F;
pub const FIRMWARE_RDP: u8 =  0xAB;
pub const FIRMWARE_DP: u8 =   0xB9;
pub const FIRMWARE_SE: u8 =   0xD8;
pub const FIRMWARE_PE: u8 =   0xDB;

pub const REG_SPI_DATA: *mut u16 = 0x040001C2 as *mut u16;
pub const REG_SPI_CNT: *mut u16 = 0x040001C0 as *mut u16;

//SPI_CNT Flags
pub const SPI_DISABLE: u16 = 0;
pub const SPI_ENABLE: u16 = 1 << 15;
pub const SPI_BYTE_MODE: u16 = 1 << 10;
pub const SPI_CONTINUOUS: u16 = 1 << 11;
pub const SPI_BUSY: u16 = 1 << 7;
//TODO: Why are there two names for the same flag???
pub const SPI_DEVICE_FIRMWARE: u16 = 1 << 8;
pub const SPI_DEVICE_NVRAM: u16 = 1 << 8;

//SPI Result flag data
pub const SPI_WORKING: u8 = 1 << 0;
pub const SPI_WRITE_ENABLED: u8 = 1 << 1;

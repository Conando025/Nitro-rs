use core::{mem::MaybeUninit, ptr::copy_nonoverlapping};

mod interrupts;

mod firmware;
use firmware::read_firmware;

mod constants;

#[derive(Clone)]
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

        let current_setting = current_settings_slot as *const PersonalData;
        copy_nonoverlapping(current_setting, PERSONAL_DATA_LOCATION, 1);
    }
}

const PERSONAL_DATA_LOCATION: *mut PersonalData = 0x2FFFC80 as *mut PersonalData;

fn swiCRC16<T>(crc: u16, data_addr: &T) -> u8 {
    todo!()
}

use core::{
    mem::transmute,
    ptr::{read_volatile, write_volatile},
};

/**# Interrupt Master Enable Register
 * When bit 0 is clear, all interrupts are masked.  When it is 1, interrupts will occur if not masked out in REG_IE.
 */
const REG_IME: *mut u32 = 0x04000208 as *mut u32;

/** Only allowed to be either 0 or 1 */
#[allow(non_camel_case_types)]
pub struct IME(u32);

pub fn enter_critical_section() -> IME {
    //TODO: Double check that this is correct because this should be one atomic instruction shouldn't it?
    unsafe {
        let old_ime = read_volatile(REG_IME);
        write_volatile(REG_IME, 0);
        transmute(old_ime)
    }
}

pub fn leave_critical_section(old_ime: IME) {
    unsafe {
        write_volatile(REG_IME, transmute(old_ime));
    }
}

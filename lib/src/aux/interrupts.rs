use core::{
    mem::transmute,
    ptr::{read_volatile, write_volatile},
};

const MAX_INTERRUPTS: usize = 25;

#[no_mangle]
#[cfg_attr(target_os = "nintendo_ds_arm9", link_section = ".itcm")]
static mut irqTable: [InterruptTableEntry; MAX_INTERRUPTS] =
    [InterruptTableEntry::DUMMY; MAX_INTERRUPTS];

#[repr(C)]
struct InterruptTableEntry {
    handler: InterruptHandler,
    mask: u32,
}

type InterruptHandler = extern "C" fn() -> ();

extern "C" fn dummy_iqr_handler() {}

impl InterruptTableEntry {
    const DUMMY: Self = InterruptTableEntry {
        handler: dummy_iqr_handler,
        mask: 0,
    };
}

/**# Interrupt Master Enable Register
 * When bit 0 is clear, all interrupts are masked.  When it is 1, interrupts will occur if not masked out in REG_IE.
 */
const REG_IME: *mut u32 = 0x04000208 as *mut u32;

/** Only allowed to be either 0 or 1 */
#[allow(non_camel_case_types)]
pub struct IME(u32);

impl IME {
    pub const ENABLE: Self = IME(1);
    pub const DISABLE: Self = IME(0);

    #[inline(always)]
    fn read() -> IME {
        unsafe { transmute(read_volatile(REG_IME)) }
    }

    #[inline(always)]
    fn write(ime: IME) {
        unsafe { write_volatile(REG_IME, transmute(ime)) }
    }
}

impl Into<u32> for IME {
    fn into(self) -> u32 {
        self.0
    }
}

#[inline]
pub fn enter_critical_section() -> IME {
    let old_ime = IME::read();
    IME::write(IME::DISABLE);
    old_ime
}

#[inline]
pub fn leave_critical_section(old_ime: IME) {
    IME::write(old_ime);
}

pub fn initialize_interrupts() {
    initialize_irq_handler();

    // The C implementaion initialized the Dummy Handler here were as we do it du

    #[cfg(target_os = "nintendo_ds_arm7")]
    {
        //TODO
    }

    IME::write(IME::ENABLE);
}

fn initialize_irq_handler() {}

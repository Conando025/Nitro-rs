use core::{ptr::read_volatile, sync::atomic::AtomicBool, sync::atomic::Ordering};

use nitro_rs::arm7::*;
//use libnds_sys::bios_registers::*;

unsafe extern "C" fn vblank_handler() {
    Wifi_Update();
}

unsafe extern "C" fn vcount_handler() {
    inputGetAndSend();
}

static EXIT_FLAG: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn power_button_cb() {
    EXIT_FLAG.store(true, core::sync::atomic::Ordering::Release);
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        readUserSettings();
        unimplemented!();
        irqInit();
        fifoInit();
        mmInstall(FIFO_MAXMOD as i32);
        // Start the RTC tracking IRQ
        initClockIRQ();

        SetYtrigger(30);
        installWifiFIFO();
        installSoundFIFO();
        installSystemFIFO();
        irqSet(IRQ_VCOUNT as u32, Some(vcount_handler));
        irqSet(IRQ_VBLANK as u32, Some(vblank_handler));
        irqEnable((IRQ_VBLANK | IRQ_VCOUNT | IRQ_NETWORK) as u32);
        setPowerButtonCB(Some(power_button_cb));

        // Keep the ARM7 mostly idle
        while !EXIT_FLAG.load(Ordering::Acquire) {
            let keyinput = read_volatile(REG_KEYINPUT);
            if (keyinput & KEY_START as u16) > 0 {
                EXIT_FLAG.store(true, Ordering::Release);
            }
        }
        return 0;
    }
}

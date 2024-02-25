use nitro_rs::arm9::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize 
{
    unsafe {
        loop {
            scanKeys();
            if (keysDown() & KEY_START) > 0
            {
                swiWaitForVBlank();
                break;
            }
        }
    }
    return 0;
}

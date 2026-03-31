use core::panic::PanicInfo;
use crate::process;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    process::exit(1);
}

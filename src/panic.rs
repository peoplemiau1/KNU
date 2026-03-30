use core::panic::PanicInfo;
use crate::io;
use crate::proc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    io::print("PANIC\n");
    proc::exit(1);
}

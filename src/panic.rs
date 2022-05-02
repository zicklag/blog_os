use core::panic::PanicInfo;

use crate::println;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!(
        "\n--------------------------------------------------------------------------------\n\
             Kernel {}\n\
             --------------------------------------------------------------------------------",
        info
    );
    loop {}
}

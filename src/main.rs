#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
mod test_runner;

mod panic;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    if cfg!(test) {
        // This is redundant is is just to make rust-analyzer happy for now.
        #[cfg(test)]
        test_main();
    } else {
        start();
    }

    loop {}
}

fn start() {
    {
        let mut writer = vga::WRITER.lock();

        writer.write_string("\nWelcome to the ");
        writer.color = vga::ColorCode::new(vga::Color::Red, vga::Color::Black);
        writer.write_string("DEAD NET");
        writer.color = vga::ColorCode::new(vga::Color::LightGray, vga::Color::Black);
        writer.write_string("... ");
        writer.color = vga::ColorCode::new(vga::Color::White, vga::Color::Black);
        writer.write_string("YOU ");
        writer.color = vga::ColorCode::new(vga::Color::LightGray, vga::Color::Black);
        writer.write_string("are in control now.");
    }

    println!();
    println!("Your wish is my command...");
}

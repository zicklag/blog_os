#![no_std]
#![no_main]

mod panic;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    {
        let mut writer = vga::WRITER.lock();

        writer.write_string("Welcome to the ");
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

    loop {}
}

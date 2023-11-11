use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::sys::_X;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Init gpio");

    let peripherals = Peripherals::take().expect("jep");
    let mut sck = PinDriver::output(peripherals.pins.gpio22).expect("scr failed");
    let mut input = PinDriver::input(peripherals.pins.gpio23).expect("input failed");

    loop {
        while input.is_high() {
            log::info!("Waiting for read to finish");
        }

        let mut val: i32 = 0;
        for _x in 1..24 {
            sck.set_high().unwrap();
            sck.set_low().unwrap();
            val = val << 1;
            if input.is_high() {
                val = val + 1;
            }
        }

        val = val ^ 0x800000;

        for _x in 1..3 {
            sck.set_high().unwrap();
            sck.set_low().unwrap();
        }

        log::info!("Value: {}", val);
        FreeRtos::delay_ms(10);
    }
}

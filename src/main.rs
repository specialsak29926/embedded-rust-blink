#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::delay::DelayMs;
use panic_halt as _;
use rp_pico::hal::{
    clocks::init_clocks_and_plls,
    gpio::Pins,
    pac,
    sio::Sio,
    watchdog::Watchdog,
    Timer,
};
use rp_pico::XOSC_CRYSTAL_FREQ;

#[entry]
fn main() -> ! {
    // Acquire access to the microcontroller's peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    // The watchdog must be set up before configuring clocks
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Configure system clocks for the RP2040 (required for timers, USB, etc.)
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // SIO is needed for GPIO and other single-cycle IO operations
    let sio = Sio::new(pac.SIO);
    // Initialize all GPIO pins
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set GPIO 25 (onboard LED) as a push-pull output
    let mut led_pin = pins.gpio25.into_push_pull_output();
    // Set up the timer peripheral for delays
    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Main loop: blink the onboard LED every second
    loop {
        // Turn the LED on
        led_pin.set_high().unwrap();
        timer.delay_ms(1000u32); // Wait 1 second
        // Turn the LED off
        led_pin.set_low().unwrap();
        timer.delay_ms(1000u32); // Wait 1 second
    }
}

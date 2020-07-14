#![no_std]
#![no_main]

mod app;
mod usb;

use panic_rtt_target as _;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let rcc = hs_probe_bsp::rcc::RCC::new(stm32ral::rcc::RCC::take().unwrap());

    let usb_global = stm32ral::otg_fs_global::OTG_FS_GLOBAL::take().unwrap();
    let usb_device = stm32ral::otg_fs_device::OTG_FS_DEVICE::take().unwrap();
    let usb_pwrclk = stm32ral::otg_fs_pwrclk::OTG_FS_PWRCLK::take().unwrap();
    let mut usb = crate::usb::USB::new(usb_global, usb_device, usb_pwrclk);

    let gpioa = hs_probe_bsp::gpio::GPIO::new(stm32ral::gpio::GPIOA::take().unwrap());
    let gpiob = hs_probe_bsp::gpio::GPIO::new(stm32ral::gpio::GPIOB::take().unwrap());
    let gpioc = hs_probe_bsp::gpio::GPIO::new(stm32ral::gpio::GPIOC::take().unwrap());
    let gpiod = hs_probe_bsp::gpio::GPIO::new(stm32ral::gpio::GPIOD::take().unwrap());
    let gpioe = hs_probe_bsp::gpio::GPIO::new(stm32ral::gpio::GPIOE::take().unwrap());
    let gpioi = hs_probe_bsp::gpio::GPIO::new(stm32ral::gpio::GPIOI::take().unwrap());

    let pins = hs_probe_bsp::gpio::Pins {
        led: gpioc.pin(10),
        tvcc_en: gpioe.pin(2),
        usart1_rx: gpiob.pin(7),
        usart2_rx: gpiod.pin(6),
        usart2_tx: gpiod.pin(5),
        spi1_clk: gpiob.pin(3),
        spi1_miso: gpiob.pin(4),
        spi1_mosi: gpiob.pin(5),
        spi2_clk: gpioi.pin(1),
        spi2_miso: gpioi.pin(2),
        spi2_mosi: gpioi.pin(3),
        usb_dm: gpioa.pin(11),
        usb_dp: gpioa.pin(12),
    };

    // Create App instance with the HAL instances
    let mut app = app::App::new(&rcc, &pins, &mut usb);

    rprintln!("Starting...");

    // Initialise application, including system peripherals
    unsafe { app.setup() };

    loop {
        // Process events
        app.poll();
    }
}

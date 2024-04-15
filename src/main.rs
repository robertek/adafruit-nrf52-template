#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals};
use embassy_nrf::usb::{Driver, InterruptHandler, vbus_detect};
use embassy_time::Timer;
use log::info;

bind_interrupts!(struct Irqs {
    USBD => InterruptHandler<peripherals::USBD>;
    POWER_CLOCK => vbus_detect::InterruptHandler;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, peripherals::USBD, vbus_detect::HardwareVbusDetect>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let driver = Driver::new(p.USBD, Irqs, vbus_detect::HardwareVbusDetect::new(Irqs));
    spawner.spawn(logger_task(driver)).unwrap();

    loop {
        info!("Tick");
        Timer::after_secs(1).await;
    }
}

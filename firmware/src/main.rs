#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::wdg::IndependentWatchdog;
use embassy_stm32::gpio::{Level, OutputOpenDrain, Speed, Input, Pull};
use embassy_time::{Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // Initialise outputs as disabled.
    let mut dcdc_disable = OutputOpenDrain::new(p.PA1, Level::High, Speed::Low);
    let mut dishy_disable = OutputOpenDrain::new(p.PA2, Level::High, Speed::Low);
    let mut router_disable = OutputOpenDrain::new(p.PA0, Level::High, Speed::Low);

    // Set up watchdog for 3 second period before reset
    let mut wdg = IndependentWatchdog::new(p.IWDG, 3_000_000);
    wdg.unleash();

    let switch_on = Input::new(p.PA3, Pull::None);

    let mut time_millis:u64 = Instant::now().as_millis();

    let mut state: u32 = 0;
    let mut state_time_millis:u64 = time_millis;

    // Always keep outputs disabled for 1 second at initial power-on
    Timer::after_millis(1000).await;

    loop {
        time_millis = Instant::now().as_millis();

        if switch_on.is_high() {
            if state == 0 {
                dcdc_disable.set_low();

                state = 1;
                state_time_millis = time_millis;
            }
            else if state == 1 && (time_millis - state_time_millis) > 1000 {
                dishy_disable.set_low();

                state = 2;
                state_time_millis = time_millis;
            }
            else if state == 2 && (time_millis - state_time_millis) > 1000 {
                router_disable.set_low();

                state = 3;
                state_time_millis = time_millis;
            }
        }
        else {
            router_disable.set_high();
            dishy_disable.set_high();
            dcdc_disable.set_high();

            state = 0;
            state_time_millis = time_millis;
        }

        Timer::after_millis(10).await;
        wdg.pet();
    }
}

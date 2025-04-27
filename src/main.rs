// fn main() {
//     println!("Hello, world!");
// }

/*******************************************************************************
使い方

ビルド：
$ cargo build

書き込み：
$ cargo embed --chip STM32H723ZG

*******************************************************************************/



#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32h7xx_hal as hal;

use hal::prelude::*;
use fugit::RateExtU32;


#[entry]
fn main() -> ! {
    use stm32h7xx_hal::stm32;
    let dp = stm32::Peripherals::take().unwrap();

    // クロック設定
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100u32.MHz()).freeze(vos, &dp.SYSCFG);

    // GPIOを有効化
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // LEDを出力に設定
    let mut led1 = gpiob.pb0.into_push_pull_output();
    let mut led2 = gpioe.pe1.into_push_pull_output();
    let mut led3 = gpiob.pb14.into_push_pull_output();

    // LED ON
    led1.set_high();
    led2.set_high();
    led3.set_high();

    loop {
        // your code goes here
        asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

        led1.toggle();
        nokin_wait(500000);
        led2.toggle();
        nokin_wait(500000);
        led3.toggle();
        nokin_wait(500000);

    /*
        // LED ON
        led1.set_high();
        led2.set_high();
        led3.set_high();
        nokin_wait(500000);

        // LED OFF
        led1.set_low();
        led2.set_low();
        led3.set_low();
        nokin_wait(500000);
    */
    }
}


/// 脳筋ウェイト
/// cnt=500000で300ミリ秒くらい
fn nokin_wait(cnt: u32) {
    let mut i = 0;
    loop {
        i += 1;
        if i >= cnt {
            break;
        }
    }
}


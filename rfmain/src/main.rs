#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate rtfm;
extern crate panic_itm;
extern crate stm32f4;
extern crate stm32f4xx_hal as hal;

#[macro_use(block)]
extern crate nb;

extern crate rflibs;

use rtfm::app;
use hal::serial;
use hal::prelude::*;
use cortex_m::iprintln;
use rflibs::sbus;


#[app(device = stm32f4::stm32f407)]
const APP: () = {
    static mut serial  : serial::Serial<stm32f4::stm32f407::USART2,(hal::gpio::gpiod::PD5<hal::gpio::Alternate<hal::gpio::AF7>>, hal::gpio::gpiod::PD6<hal::gpio::Alternate<hal::gpio::AF7>>)> = ();
    static mut itm : stm32f4::stm32f407::ITM = ();
    static mut sbus_state: sbus::SbusReadState = ();

    #[init]
    fn init() {
        // injected by app macro :
        //    core : rtfm::Peripherals
        //    device : stm32f4::stm32f407::Peripherals

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // USART2 at PD5 (TX) and PD6(RX)
        let gpiod = device.GPIOD.split();
        let txpin = gpiod.pd5.into_alternate_af7();
        let rxpin = gpiod.pd6.into_alternate_af7();
        let config = serial::config::Config::default()
            .baudrate(100_000.bps())
            .parity_even()
            .wordlength_9()
            .stopbits(serial::config::StopBits::STOP2);
        let mut serial_ = serial::Serial::usart2(device.USART2, (txpin, rxpin), config, clocks).unwrap();
        serial_.listen(serial::Event::Rxne);

        serial = serial_;
	itm = core.ITM;
        sbus_state = sbus::SbusReadState::default();
    }

    #[interrupt(resources=[serial,sbus_state], spawn=[process], priority=2)]
    fn USART2() {
        if resources.serial.is_idle() {
            sbus::process_idle(&mut resources.sbus_state);
        }
        let received = block!(resources.serial.read());
        match received {
            Ok(c) => {
                let complete = sbus::process_char(&mut resources.sbus_state, c);
                if complete {
                    spawn.process().unwrap();
                }
            },
            Err(_e) => {
                sbus::process_idle(&mut resources.sbus_state);
            },
        }
    }

    #[task(resources=[itm,sbus_state])]
    fn process() {
        let frame = resources.sbus_state.lock(|sbus_state| {
            sbus_state.frame.clone()
        });
        iprintln!(&mut resources.itm.stim[0], "{} {}", frame.channels[0],  frame.channels[1]);
    }

    #[idle(resources=[itm,serial])]
    fn idle() -> ! {
        loop {
        }
    }

    // Spare innterrupt handler used to dispatch software tasks
    extern "C" {
        fn USART1();
    }
};

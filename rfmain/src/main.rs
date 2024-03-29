#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate rtfm;
extern crate panic_itm;
extern crate stm32f4;
extern crate stm32f4xx_hal as hal;
extern crate embedded_hal;

#[macro_use(block)]
extern crate nb;

extern crate rflibs;
extern crate heapless;

use rtfm::app;
use hal::serial;
use hal::gpio;
use hal::gpio::gpiod;
use hal::prelude::*;
use rflibs::sbus;
use heapless::consts::U32;

pub mod uarts;
pub mod serialui;


#[app(device = stm32f4::stm32f407)]
const APP: () = {
    static mut sbus_state: sbus::ReadState = ();
    static mut sbus_frame: sbus::Frame = ();

    static mut serial_ui : serialui::SerialUi<uarts::Serial3,U32,U32> = ();

    static mut led_green:  gpiod::PD12<gpio::Output<gpio::PushPull>> = ();
    static mut led_orange: gpiod::PD13<gpio::Output<gpio::PushPull>> = ();
    static mut led_red:    gpiod::PD14<gpio::Output<gpio::PushPull>> = ();
    static mut led_blue:   gpiod::PD15<gpio::Output<gpio::PushPull>> = ();

    static mut serial2 : uarts::Serial2 = ();
    static mut itm : stm32f4::stm32f407::ITM = ();

    #[init]
    fn init() {
        // injected by app macro :
        //    core : rtfm::Peripherals
        //    device : stm32f4::stm32f407::Peripherals

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
        let gpioc = device.GPIOC.split();
        let gpiod = device.GPIOD.split();

        // USART2 at PD5 (TX) and PD6(RX)
        let config = serial::config::Config::default()
            .baudrate(100_000.bps())
            .parity_even()
            .wordlength_9()
            .stopbits(serial::config::StopBits::STOP2);

        let txpin = gpiod.pd5.into_alternate_af7();
        let rxpin = gpiod.pd6.into_alternate_af7();
        let mut serial2_ = serial::Serial::usart2(device.USART2, (txpin, rxpin), config, clocks).unwrap();
        serial2_.listen(serial::Event::Rxne);

        // USART3 at PC10 (TX) and PC11 (RX)
        let config = serial::config::Config::default()
            .baudrate(19_200.bps())
            .parity_even()
            .wordlength_9()
            .stopbits(serial::config::StopBits::STOP2);

        let txpin = gpioc.pc10.into_alternate_af7();
        let rxpin = gpioc.pc11.into_alternate_af7();
        let mut serial3_ = serial::Serial::usart3(device.USART3, (txpin, rxpin), config, clocks).unwrap();
        serial3_.listen(serial::Event::Txe);
        serial3_.listen(serial::Event::Rxne);

        led_green = gpiod.pd12.into_push_pull_output();
        led_orange = gpiod.pd13.into_push_pull_output();
        led_red = gpiod.pd14.into_push_pull_output();
        led_blue = gpiod.pd15.into_push_pull_output();
        serial2 = serial2_;
        itm = core.ITM;
        sbus_state = sbus::ReadState::default();
        sbus_frame = sbus::Frame::default();
        serial_ui = serialui::SerialUi::init(serial3_);
    }

    #[interrupt(resources=[serial2,sbus_state,sbus_frame,led_green], spawn=[], priority=2)]
    fn USART2() {
        resources.led_green.set_high();
        match rx_sbus(resources.serial2, &mut resources.sbus_state) {
            Some(frame) => {
                *resources.sbus_frame = frame;
            },
            None => ()
        }
        resources.led_green.set_low();
    }

    #[interrupt(resources=[serial_ui,led_blue], spawn=[], priority=2)]
    fn USART3() {
        resources.led_blue.set_high();
        resources.serial_ui.on_event();
        resources.led_blue.set_low();
    }

    #[idle(resources=[led_red])]
    fn idle() -> ! {
        loop {
            resources.led_red.set_high();
            resources.led_red.set_low();
        }
    }

    // Spare innterrupt handler used to dispatch software tasks
    extern "C" {
        fn USART1();
    }
};

fn rx_sbus(s: &mut uarts::SerialRW, sbus_state: &mut sbus::ReadState) -> Option<sbus::Frame> {
  if s.is_idle() {
    sbus::process_idle(sbus_state);
  }
  let received = block!(s.read());
  match received {
    Ok(c) => {
      let complete = sbus::process_char(sbus_state, c);
      if complete {
        Some(sbus_state.frame.clone())
      } else {
        None
      }
    },
    Err(_e) => {
      sbus::process_idle(sbus_state);
      None
    },
  }
}

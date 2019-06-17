
use hal::serial;

pub type Serial1 = serial::Serial<
   stm32f4::stm32f407::USART1,
   (  hal::gpio::gpioa::PA9<hal::gpio::Alternate<hal::gpio::AF7>>,
      hal::gpio::gpioa::PA10<hal::gpio::Alternate<hal::gpio::AF7>>
   )
>;

pub type Serial2 = serial::Serial<
   stm32f4::stm32f407::USART2,
   (  hal::gpio::gpiod::PD5<hal::gpio::Alternate<hal::gpio::AF7>>,
      hal::gpio::gpiod::PD6<hal::gpio::Alternate<hal::gpio::AF7>>
   )
>;

pub type Serial3 = serial::Serial
   <stm32f4::stm32f407::USART3,
   ( hal::gpio::gpioc::PC10<hal::gpio::Alternate<hal::gpio::AF7>>,
     hal::gpio::gpioc::PC11<hal::gpio::Alternate<hal::gpio::AF7>>
   )
 >;

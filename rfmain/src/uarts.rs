
use hal::serial;

pub trait SerialRW {
  fn is_idle(& self) -> bool;
  fn is_txe(& self) -> bool;
  fn is_rxne(& self) -> bool;
  fn read(&mut self) -> nb::Result<u8, serial::Error>;
  fn write(&mut self, word: u8) -> nb::Result<(),serial::Error>;
}

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

impl SerialRW for Serial1 {
  fn is_idle(&self) -> bool { Serial1::is_idle(self) }
  fn is_txe(&self) -> bool { Serial1::is_txe(self) }
  fn is_rxne(&self) -> bool { Serial1::is_rxne(self) }
  fn read(&mut self) -> nb::Result<u8, serial::Error> {
    embedded_hal::serial::Read::read(self)
  }
  fn write(&mut self, word: u8) -> nb::Result<(),serial::Error> {
    embedded_hal::serial::Write::write(self, word)
  }
}

impl SerialRW for Serial2 {
  fn is_idle(&self) -> bool { Serial2::is_idle(self) }
  fn is_txe(&self) -> bool { Serial2::is_txe(self) }
  fn is_rxne(&self) -> bool { Serial2::is_rxne(self) }
  fn read(&mut self) -> nb::Result<u8, serial::Error> {
    embedded_hal::serial::Read::read(self)
  }
  fn write(&mut self, word: u8) -> nb::Result<(),serial::Error> {
    embedded_hal::serial::Write::write(self, word)
  }
}

impl SerialRW for Serial3 {
  fn is_idle(&self) -> bool { Serial3::is_idle(self) }
  fn is_txe(&self) -> bool { Serial3::is_txe(self) }
  fn is_rxne(&self) -> bool { Serial3::is_rxne(self) }
  fn read(&mut self) -> nb::Result<u8, serial::Error> {
    embedded_hal::serial::Read::read(self)
  }
  fn write(&mut self, word: u8) -> nb::Result<(),serial::Error> {
    embedded_hal::serial::Write::write(self, word)
  }
}

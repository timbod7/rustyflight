use hal::serial;
use uarts::SerialRW;
use rflibs::command_buffer;

use heapless::spsc::Queue;

pub struct SerialUi<UART,TXSIZE,RXSIZE>
  where UART : SerialRW,
        TXSIZE: heapless::ArrayLength<u8>,
        RXSIZE: heapless::ArrayLength<u8>
{
  pub uart : UART,
  tx_q : Queue<u8,TXSIZE>,
  rx_buf: command_buffer::State<RXSIZE>,
}

impl <UART,TXSIZE,RXSIZE> SerialUi<UART,TXSIZE,RXSIZE>
  where UART : SerialRW,
        TXSIZE : heapless::ArrayLength<u8>,
        RXSIZE: heapless::ArrayLength<u8>
{
  pub fn init(uart: UART) -> SerialUi<UART,TXSIZE,RXSIZE> {
    SerialUi{
      uart:uart,
      tx_q: Queue::new(),
      rx_buf: command_buffer::State::init()
    }
  }
  // Write a character to the serial port
  // Queue it if the port is busy,
  // return WouldBlock if the queue is full
  pub fn write(&mut self, c: u8) -> nb::Result<(),serial::Error> {
    if self.uart.is_txe() {
      self.uart.write(c)
    } else {
      match self.tx_q.enqueue(c) {
        Ok(()) => Ok(()),
        Err(_) => Err(nb::Error::WouldBlock)
      }
    }
  }

  // Call this to handle a serial interrupt
  pub fn on_event(&mut self) {
    if self.uart.is_rxne() {
      self.on_rxne()
    }
    if self.uart.is_txe() {
      self.on_txe()
    }
  }

  fn on_txe(&mut self) {
   match self.tx_q.dequeue() {
     Some(c) => self.uart.write(c).unwrap(),
     None => ()
   }
  }

  fn on_rxne(&mut self) {
    match self.uart.read() {
      Ok(c) => self.rx_buf.process(c as char),
      Err(_) => ()
    }
  }
}

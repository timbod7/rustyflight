use core::str;
use heapless::Vec;
use typenum::Unsigned;

pub struct State<USIZE>
  where USIZE: heapless::ArrayLength<u8>
{
  vec: Vec<u8, USIZE>,
  insert: usize
}


impl<USIZE> State<USIZE>
  where USIZE: heapless::ArrayLength<u8>
{
  pub fn process(&mut self, c:char) -> () {
    if c == '\x01' {
      self.mvstart();
    } else if c == '\x02' {
      self.mvleft();
    } else if c == '\x05' {
      self.mvend();
    } else if c == '\x06' {
      self.mvright();
    } else if c == '\x08' {
      self.backspace();
    } else if c == '\x0b' {
      self.deleteend();
    } else if c == '\x7f' {
      self.delete();
    } else {
      self.insert(c);
    }
  }

  pub fn processstr(&mut self, s : &str) {
    for c in s.chars() {
      self.process(c);
    }
  }

  fn insert(&mut self, c:char) -> () {
    // Ignore unicode
    if c.len_utf8() != 1 {
      return;
    }
    let mut b = [0; 4];
    c.encode_utf8(&mut b);

    match self.vec.push(0) {
      Err(_) => return,
      Ok(_) => ()
    }
    for i in (self.insert..self.vec.len()-1).rev() {
      self.vec[i+1] = self.vec[i];
    }
    self.vec[self.insert] = b[0];
    self.insert += 1;
  }

   fn mvstart(&mut self) -> () {
     self.insert = 0;
   }
 
   fn mvend(&mut self) -> () {
     self.insert = self.vec.len();
   }
 
   fn deleteend(&mut self) -> () {
     let _ = self.vec.resize(self.insert, 0);
   }
 
   fn mvleft(&mut self) -> () {
     if self.insert > 0 {
       self.insert = self.insert - 1;
     }
   }
 
   fn mvright(&mut self) -> () {
     if self.insert < self.vec.len() {
       self.insert = self.insert + 1;
     }
   }
 
   fn delete(&mut self) -> () {
     if self.insert < self.vec.len() {
       for i in self.insert..self.vec.len()-1 {
         self.vec[i] = self.vec[i+1];
       }
       self.vec.pop();
     }
   }
 
   fn backspace(&mut self) -> () {
     if self.insert > 0 {
       self.insert -= 1;
       self.delete();
     }
   }
 
   pub fn content(&self) -> &str {
     str::from_utf8(&self.vec[0..self.vec.len()]).unwrap()
   }
 
   pub fn init() -> State<USIZE> {
     State {
       vec: Vec::new(),
       insert: 0
     }
   }
}


#[cfg(feature="std")]
#[cfg(test)]
mod tests {
  use super::*;
  use heapless::consts::U128 as SZ;

  fn insert<USIZE>(cs: &mut State<USIZE>, s : &str)
    where USIZE: heapless::ArrayLength<u8>
  {
    for c in s.chars() {
      cs.insert(c);
    }
  }

  #[test]
  fn starts_empty() {
      let cs : State<SZ> = State::init();
      assert_eq!(cs.content(), "");
  }

  #[test]
  fn move_left() {
    let mut cs : State<SZ> = State::init();
    insert(&mut cs, "abcd");
    cs.mvleft();
    cs.mvleft();
    cs.insert('_');
    assert_eq!(cs.content(), "ab_cd");
    for _i in 0..5 {
      cs.mvleft();
    }
    cs.insert('_');
    assert_eq!(cs.content(), "_ab_cd");
  }

  #[test]
  fn move_right() {
    let mut cs : State<SZ> = State::init();
    insert(&mut cs, "abcd");
    cs.mvleft();
    cs.mvleft();
    cs.mvleft();
    cs.mvright();
    cs.insert('_');
    assert_eq!(cs.content(), "ab_cd");
    for _i in 0..5 {
      cs.mvright();
    }
    cs.insert('_');
    assert_eq!(cs.content(), "ab_cd_");
  }

  #[test]
  fn repeated_inserts() {
    let mut cs : State<SZ> = State::init();
    insert(&mut cs, "abc");
    assert_eq!(cs.content(), "abc");

    for _i in 0..SZ::to_usize()+10 {
      cs.insert('_');
    }
    let mut expected = String::from("abc");
    for _i in 0..SZ::to_usize() - 3 {
      expected.push('_');
    }
    assert_eq!(cs.content(), expected);
  }

  #[test]
  fn deletes() {
    let mut cs : State<SZ> = State::init();
    insert(&mut cs, "abcd");
    cs.mvleft();
    cs.mvleft();
    cs.delete();
    assert_eq!(cs.content(), "abd");
    for _i in 0..5 {
      cs.delete();
    }
    assert_eq!(cs.content(), "ab");
  }

  #[test]
  fn backspaces() {
    let mut cs : State<SZ> = State::init();
    insert(&mut cs, "abcd");
    cs.backspace();
    assert_eq!(cs.content(), "abc");
    cs.mvleft();
    cs.mvleft();
    cs.backspace();
    assert_eq!(cs.content(), "bc");
    cs.backspace();
    assert_eq!(cs.content(), "bc");
  }

  #[test]
  fn split() {
    let mut cs : State <SZ> = State::init();
    insert(&mut cs, "load from file");
    let mut iter = cs.content().split_whitespace();
    assert_eq!(Some("load"), iter.next());
    assert_eq!(Some("from"), iter.next());
    assert_eq!(Some("file"), iter.next());
    assert_eq!(None, iter.next());
  }
}

use core::str;

const BUFFER_SIZE:usize = 100;

pub struct BufferState {
  array: [u8; BUFFER_SIZE],
  length: usize,
  insert: usize
}


impl BufferState {
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

    if self.length < self.array.len() {
      self.array[self.insert..self.length+1].rotate_right(1);
      self.array[self.insert] = b[0];
      self.insert += 1;
      self.length += 1
    }
  }

  fn mvstart(&mut self) -> () {
    self.insert = 0;
  }

  fn mvend(&mut self) -> () {
    self.insert = self.length;
  }

  fn deleteend(&mut self) -> () {
    self.length = self.insert;
  }

  fn mvleft(&mut self) -> () {
    if self.insert > 0 {
      self.insert = self.insert - 1;
    }
  }

  fn mvright(&mut self) -> () {
    if self.insert < self.length {
      self.insert = self.insert + 1;
    }
  }

  fn delete(&mut self) -> () {
    if self.insert < self.length {
      self.array[self.insert..self.length].rotate_left(1);
      self.length -= 1;
    }
  }

  fn backspace(&mut self) -> () {
    if self.insert > 0 && self.insert <= self.length {
      if self.insert != self.length {
        self.array[self.insert-1..self.length].rotate_left(1);
      }
      self.insert -= 1;
      self.length -= 1;
    }
  }


  pub fn content(&self) -> &str {
    str::from_utf8(&self.array[0..self.length]).unwrap()
  }

  pub fn init() -> BufferState {
    BufferState {
      array: [0;BUFFER_SIZE],
      length: 0,
      insert: 0
    }
  }
}


#[cfg(feature="std")]
#[cfg(test)]
mod tests {
  use super::*;

  fn insert(cs: &mut BufferState, s : &str) {
    for c in s.chars() {
      cs.insert(c);
    }
  }

  #[test]
  fn starts_empty() {
      let cs = BufferState::init();
      assert_eq!(cs.content(), "");
  }

  #[test]
  fn move_left() {
    let mut cs = BufferState::init();
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
    let mut cs = BufferState::init();
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
    let mut cs = BufferState::init();
    insert(&mut cs, "abc");
    assert_eq!(cs.content(), "abc");

    for _i in 0..BUFFER_SIZE+10 {
      cs.insert('_');
    }
    let mut expected = String::from("abc");
    for _i in 0..BUFFER_SIZE - 3 {
      expected.push('_');
    }
    assert_eq!(cs.content(), expected);
  }

  #[test]
  fn deletes() {
    let mut cs = BufferState::init();
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
    let mut cs = BufferState::init();
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
    let mut cs = BufferState::init();
    insert(&mut cs, "load from file");
    let mut iter = cs.content().split_whitespace();
    assert_eq!(Some("load"), iter.next());
    assert_eq!(Some("from"), iter.next());
    assert_eq!(Some("file"), iter.next());
    assert_eq!(None, iter.next());
  }
}

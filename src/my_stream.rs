use std::process::{Child, ChildStdout};
use std::io::Read;
use std::iter::FromIterator;
use std::str;
use std::marker::Sized;


pub struct LineCodec {
  decoding_head: bool,
  internal_buf: Vec<u8>,
  stdout: ChildStdout
}

impl LineCodec {
  fn decode(&mut self) -> Option<String> {
    let mut new_bytes: Vec<u8> = Vec::new();
    self.stdout.read(&mut new_bytes).unwrap();
    self.internal_buf.extend(new_bytes.iter());
    // Find the position of the next newline character
    let pos = self.internal_buf.iter().position(|b| *b == b'\n');

    pos.map(|n| {
      let line: Vec<u8> = self.internal_buf.drain(..n).collect();

      //drain the newline character
      self.internal_buf.remove(0);

      str::from_utf8(&line).unwrap().to_string()
    })
  }

  fn flush(&mut self, handle: i32) {
  }
}
use std::process::{ChildStdout};
use std::sync::mpsc::Sender;
use std::io::Read;
use std::str;

pub struct LineCodec {
  pub internal_buf: Vec<u8>,
  pub stdout: ChildStdout,
  pub handle: Sender<String>
}

impl LineCodec {
  pub fn decode(&mut self) -> Option<String> {
    let mut new_bytes: Vec<u8> = vec![0; 100];
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

  pub fn flush(&mut self, msg: String) {
    self.handle.send(msg).unwrap()
  }
}
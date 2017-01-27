mod my_stream;
use std::process::{Command, Stdio, Child};
use std::str;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

fn run_from_args(arg_str: &str) -> Child {
  let mut arg_iter = arg_str.split(" ");
  let base_cmd = arg_iter.nth(0).expect("requires a command");
  let cmd_opts = arg_iter.collect::<Vec<_>>();

  Command::new(base_cmd)
    .args(&cmd_opts)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("must provide a valid command")
}

fn stream_from_thread(cmd: &str) {
  let child = run_from_args(cmd);
  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
  let thread_tx = tx.clone();


  thread::spawn(move || {
    let mut lc = my_stream::LineCodec {
      internal_buf: Vec::new(),
      stdout: child.stdout.unwrap(),
      handle: thread_tx
    };

    loop {
      lc.decode().map(|msg| lc.flush(msg));
    }
  });

  loop {
    if let Ok(x) = rx.recv() {
      println!("{}", x);
    }
  }
}

fn main() {
  let cmd = "ping google.com";
  stream_from_thread(cmd)
}
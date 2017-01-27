mod my_stream;
use std::process::{Command, Stdio, Child};
use std::io::Read;
use std::str;


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

fn spawn_cmd_in_thread(cmd: &str) -> String {
  use std::sync::mpsc::{Sender, Receiver};
  use std::sync::mpsc;
  use std::thread;
  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
  let thread_tx = tx.clone();
  let moved_cmd = cmd.to_string();

  thread::spawn(move || {
    let child = run_from_args(&moved_cmd);
    let res = stdout_as_str(child);
    thread_tx.send(res).unwrap();
  });

  rx.recv().unwrap()
}

fn main() {
  let cmd = "ls -lh /tmp";
  let run_in_another_thread = spawn_cmd_in_thread(cmd);
  println!("{}", run_in_another_thread);
}



fn stdout_as_str(child: Child,) -> String {
  let mut stdout = child.stdout.unwrap();
  let mut output: Vec<u8> = vec![];
  //for some reason it seems the vec must be initialized with a value.
  let mut buf = &mut vec![0];
  let mut still_reading = true;

  while still_reading {
    match stdout.read(buf) {
      Ok(byte_ln) => {
        if byte_ln > 0 {
          output.extend(buf.iter());
        } else {
          still_reading = false;
        }
      }
      Err(_) => {}
    }
  }

  str::from_utf8(&output).unwrap().to_string()
}
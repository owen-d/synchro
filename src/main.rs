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

// fn merge_processes() {}
// fn to read bytes from stdout into a Vec
// fn to peek @ a vec, either removing through a newline, or not modifying the vec


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

fn main() {
  let child = run_from_args("find /tmp/");
  let output = stdout_as_str(child);
  println!("{}", output);
}
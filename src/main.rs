// use std::error::Error;
// use std::io::prelude::*;
use std::process::{Command, Stdio, Child};
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

fn main() {
  let child = run_from_args("ls -l -h /tmp");


  let output = child
    .wait_with_output()
    .expect("failed to wait on child");

  println!("{}", str::from_utf8(&output.stdout).expect("unable to unwrap"));
}

// fn main() {
//     // Spawn the `wc` command
//     let process = match Command::new("wc")
//                                 .stdin(Stdio::piped())
//                                 .stdout(Stdio::piped())
//                                 .spawn() {
//         Err(why) => panic!("couldn't spawn wc: {}", why.description()),
//         Ok(process) => process,
//     };

//     // Write a string to the `stdin` of `wc`.
//     //
//     // `stdin` has type `Option<ChildStdin>`, but since we know this instance
//     // must have one, we can directly `unwrap` it.
//     match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
//         Err(why) => panic!("couldn't write to wc stdin: {}",
//                            why.description()),
//         Ok(_) => println!("sent pangram to wc"),
//     }

//     // Because `stdin` does not live after the above calls, it is `drop`ed,
//     // and the pipe is closed.
//     //
//     // This is very important, otherwise `wc` wouldn't start processing the
//     // input we just sent.

//     // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
//     let mut s = String::new();
//     match process.stdout.unwrap().read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read wc stdout: {}",
//                            why.description()),
//         Ok(_) => print!("wc responded with:\n{}", s),
//     }
// }
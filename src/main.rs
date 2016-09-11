extern crate notify;

use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;

fn main() {
    exec_licecap();
    let pid = get_pid();
    println!("licecap pid is{}", pid);
    println!("monitor: {}", monitor_gif());
    kill_licecap();
}

fn exec_licecap() {
    let output = Command::new("open")
                     .arg("-a")
                     .arg("licecap")
                     .output()
                     .expect("failde to execute process");
    println!("exec licecap: {}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

fn get_pid() -> String {
    let ps_process = Command::new("ps")
                         .arg("aux")
                         .stdout(Stdio::piped())
                         .output()
                         .expect("failde to execute process");

    let grep_lice_cap_process = match Command::new("grep")
                                          .arg("licecap")
                                          .stdin(Stdio::piped())
                                          .stdout(Stdio::piped())
                                          .spawn() {
        Err(why) => panic!("couldn't spawn grep lice cap: {}", Error::description(&why)),
        Ok(process) => process,
    };
    let grep_grep_process = match Command::new("grep")
                                      .arg("-v")
                                      .arg("grep")
                                      .stdin(Stdio::piped())
                                      .stdout(Stdio::piped())
                                      .spawn() {
        Err(why) => panic!("couldn't spawn grep lice cap: {}", Error::description(&why)),
        Ok(process) => process,
    };
    let awk_process = match Command::new("awk")
                                .arg("{ print $2 }")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn awk: {}", Error::description(&why)),
        Ok(process) => process,
    };

    grep_lice_cap_process.stdin.unwrap().write_all(&ps_process.stdout);
    let mut str = String::new();
    grep_grep_process.stdin
                     .unwrap()
                     .write_all(match grep_lice_cap_process.stdout
                                                           .unwrap()
                                                           .read_to_string(&mut str) {
                         Err(why) => {
                             panic!("couldn't read grep_grep stdout: {}",
                                    Error::description(&why))
                         }
                         Ok(_) => str.as_bytes(),
                     });

    let mut str2 = String::new();
    awk_process.stdin
               .unwrap()
               .write_all(match grep_grep_process.stdout
                                                 .unwrap()
                                                 .read_to_string(&mut str2) {
                   Err(why) => {
                       panic!("couldn't read grep_grep stdout: {}",
                              Error::description(&why))
                   }
                   Ok(_) => str.as_bytes(),
               });

    let mut s = String::new();
    // match grep_grep_process.stdout.unwrap().read_to_string(&mut s) {
    // match awk_process.stdout.unwrap().read_to_string(&mut s) {
    return match awk_process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", Error::description(&why)),
        Ok(_) => s,
    };
}
fn monitor_gif() -> String {
    let (tx, rx) = channel();
    let mut w: Result<RecommendedWatcher, notify::Error> = Watcher::new(tx);

    match w {
        Ok(mut watcher) => {
            watcher.watch("/Users/koji/Desktop");
            loop {
                match rx.recv() {
                    Ok(notify::Event{path: Some(path), op: Ok(op)}) => {
                        if op.contains(notify::op::WRITE) && op.contains(notify::op::CHMOD) {
                            return path.as_os_str().to_str().unwrap().to_string();
                        }
                        println!("{:?} {:?}", op, path)
                    }
                    Ok(event) => println!("broken event: {:?}", event),
                    _ => println!("Recv."),
                }
            }
        }
        Err(_) => println!("Error"),

    }
    return String::from("finish");
}

fn kill_licecap() {
    let output = Command::new("killall")
                     .arg("licecap")
                     .output()
                     .expect("failde to execute process");
    println!("exec licecap: {}", String::from_utf8_lossy(&output.stdout));
    println!("error{}", String::from_utf8_lossy(&output.stderr));
}

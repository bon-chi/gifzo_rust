extern crate multipart;
extern crate hyper;
extern crate url;
extern crate mime;
extern crate mime_guess;
extern crate notify;
extern crate clipboard;
//
use multipart::client;
use hyper::client::request;
// use multipart::client::lazy::Multipart;
// use multipart::client::HttpStream::Request;
use std::path::Path;
use url::Url;
use mime::Mime;
use std::fs::File;
// use std::error::Error;
// use std::io::prelude::*;
use std::process::{Command, Stdio};
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use clipboard::ClipboardContext;

fn main() {
    exec_licecap();
    // let pid = get_pid();
    // println!("licecap pid is{}", pid);
    let (filename, filepath_str) = monitor_gif();
    // let s_slice: &str = filename.as_str();
    // println!("monitor: {}", filename);
    kill_licecap();
    println!("filename ==== {}", filename);
    let filename2 = filename.clone();
    post_gif(filename, filepath_str);
    // post_gif2();
    open_browser(filename2);
}

// fn post_gif() {
fn post_gif(filename: String, filepath_str: String) {
    println!("filepath_str == {}", filepath_str);
    let url = hyper::Url::parse("http://localhost:3000").unwrap();
    let request = request::Request::new(hyper::method::Method::Post, url).unwrap();
    let mut multipart = client::Multipart::from_request(request).unwrap();
    let filepath = Path::new("lorem_ipsum.txt");
    let filepath2 = Path::new("lorem_ipsum.txt");
    let filepath3 = Path::new("main2.rs");
    // let filepath4 = Path::new("giphy.gif");
    // let s_slice: &str = filename.as_str();
    // let filepath_str: &str = format!("{}{}", "~/Desktop/", s_slice);
    let filepath_str: &str = filepath_str.as_str();
    let filepath4 = Path::new(filepath_str);
    // println!("{:?}", mime_guess::guess_mime_type(filepath3));
    // println!("{:?}",
    //          filepath3.file_name().and_then(|filename| filename.to_str()));
    // let filepath = Path::new("./src/main2.rs");
    let mut file = File::open(filepath4).unwrap();
    let name: &str = "main3.rs";
    let file_name: Option<&str> = Some("lorem_ipsum.txt");
    // let file_name4: Option<&str> = Some("giphy.gif");
    let s_slice: &str = filename.as_str();
    let file_name4: Option<&str> = Some(s_slice);
    let mime: Option<hyper::mime::Mime> = Some("text/plain".parse().unwrap());
    // let mime: Option<hyper::mime::Mime> = Some("text/plain".parse().unwrap());
    println!("{:?}", mime);
    // multipart.write_stream(name, &mut file, file_name, mime);
    // let re = multipart.write_stream(name, &mut file, file_name, mime);
    // multipart.write_file("main2.rs", filepath);
    // let re = multipart.write_file("main4.rs", filepath);
    multipart.write_text("text_name",
                         filepath4.file_name().unwrap().to_str().unwrap());
    multipart.write_file("file", filepath4);
    // multipart.write_stream("file", &mut file, file_name4, mime);
    multipart.send();
    // match re {
    //     Ok(_) => println!("OKo"),
    //     Err(_) => println!("err"),
    // }
    // let mut resp = try!(multipart.send());
    // multipart.send();

    // let result = multipart.client_request(&hyper::client::Client::new(), "http://localhost:3000/");
    // match result {
    //     Ok(_) => println!("foo"),
    //     Err(why) => println!("{}", why),
    // };
    println!("hoge");
    // println!("{}" result);
    // let client = hyper::client::Client::new().expect("Failed to create a Client");
}

fn post_gif2() {
    let url = hyper::Url::parse("http://localhost:3000").unwrap();
    let mut multipart = multipart::client::lazy::Multipart::new();
    multipart.add_text("gifname", "hogehogehgoe");
    multipart.add_file("main2.rs", Path::new("./src/main2.rs"));
    let result = multipart.client_request(&hyper::client::Client::new(), url);
    // let client = hyper::client::Client::new().expect("Failed to create a Client")
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

// fn get_pid() -> String {
//     let ps_process = Command::new("ps")
//                          .arg("aux")
//                          .stdout(Stdio::piped())
//                          .output()
//                          .expect("failde to execute process");
//
//     let grep_lice_cap_process = match Command::new("grep")
//                                           .arg("licecap")
//                                           .stdin(Stdio::piped())
//                                           .stdout(Stdio::piped())
//                                           .spawn() {
//         Err(why) => panic!("couldn't spawn grep lice cap: {}", Error::description(&why)),
//         Ok(process) => process,
//     };
//     let grep_grep_process = match Command::new("grep")
//                                       .arg("-v")
//                                       .arg("grep")
//                                       .stdin(Stdio::piped())
//                                       .stdout(Stdio::piped())
//                                       .spawn() {
//         Err(why) => panic!("couldn't spawn grep lice cap: {}", Error::description(&why)),
//         Ok(process) => process,
//     };
//     let awk_process = match Command::new("awk")
//                                 .arg("{ print $2 }")
//                                 .stdin(Stdio::piped())
//                                 .stdout(Stdio::piped())
//                                 .spawn() {
//         Err(why) => panic!("couldn't spawn awk: {}", Error::description(&why)),
//         Ok(process) => process,
//     };
//
//     grep_lice_cap_process.stdin.unwrap().write_all(&ps_process.stdout);
//     let mut str = String::new();
//     grep_grep_process.stdin
//                      .unwrap()
//                      .write_all(match grep_lice_cap_process.stdout
//                                                            .unwrap()
//                                                            .read_to_string(&mut str) {
//                          Err(why) => {
//                              panic!("couldn't read grep_grep stdout: {}",
//                                     Error::description(&why))
//                          }
//                          Ok(_) => str.as_bytes(),
//                      });
//
//     let mut str2 = String::new();
//     awk_process.stdin
//                .unwrap()
//                .write_all(match grep_grep_process.stdout
//                                                  .unwrap()
//                                                  .read_to_string(&mut str2) {
//                    Err(why) => {
//                        panic!("couldn't read grep_grep stdout: {}",
//                               Error::description(&why))
//                    }
//                    Ok(_) => str.as_bytes(),
//                });
//
//     let mut s = String::new();
//     // match grep_grep_process.stdout.unwrap().read_to_string(&mut s) {
//     // match awk_process.stdout.unwrap().read_to_string(&mut s) {
//     return match awk_process.stdout.unwrap().read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read wc stdout: {}", Error::description(&why)),
//         Ok(_) => s,
//     };
// }
fn monitor_gif() -> (String, String) {
    let (tx, rx) = channel();
    let mut w: Result<RecommendedWatcher, notify::Error> = Watcher::new(tx);

    match w {
        Ok(mut watcher) => {
            watcher.watch("/Users/koji/Desktop");
            loop {
                match rx.recv() {
                    Ok(notify::Event { path: Some(path), op: Ok(op) }) => {
                        if op.contains(notify::op::WRITE) && op.contains(notify::op::CHMOD) {
                            // return path.as_os_str().to_str().unwrap().to_string();
                            return (path.file_name()
                                        .unwrap()
                                        .to_os_string()
                                        .to_str()
                                        .unwrap()
                                        .to_string(),
                                    path.as_os_str().to_str().unwrap().to_string());
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
    return (String::from("finish"), String::from("finish"));
}

fn kill_licecap() {
    let output = Command::new("killall")
                     .arg("licecap")
                     .output()
                     .expect("failde to execute process");
    println!("exec licecap: {}", String::from_utf8_lossy(&output.stdout));
    println!("error{}", String::from_utf8_lossy(&output.stderr));
}

fn open_browser(filename: String) {
    let address = format!("{}{}", "http://localhost:3000/gifs/", filename);
    let address2 = format!("{}{}", "http://localhost:3000/gifs/", filename);
    println!("address is === {}", address);
    let output = Command::new("open")
                     .arg(address)
                     .output()
                     .expect("failde to execute process");
    println!("opne browser: {}", String::from_utf8_lossy(&output.stdout));
    println!("error{}", String::from_utf8_lossy(&output.stderr));

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(address2);
}

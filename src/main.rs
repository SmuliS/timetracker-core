use std::io::Read;
use std::{error::Error, os::unix::net::UnixListener, path::Path, process, thread};
use std::{fmt, os::unix::net::UnixStream};

use app::{create_app, SOCKET};
use clap::ArgMatches;

mod app;

fn main() {
    let args = create_app().get_matches();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_socket(mut stream: UnixStream) -> () {
    let mut res = String::new();
    stream.read_to_string(&mut res).unwrap();
    println!("Received: {}", res);
}

fn run(args: ArgMatches) -> Result<(), Box<dyn Error>> {
    let socket = get_socket(&args)?;
    for stream in socket.incoming() {
        match stream {
            Ok(stream) => thread::spawn(|| handle_socket(stream)),
            Err(e) => break,
        };
    }
    Ok(())
}

#[derive(Debug)]
struct SocketError;

impl SocketError {
    fn new(_e: &::std::io::Error) -> Box<Self> {
        Box::new(SocketError {})
    }
}

impl Error for SocketError {}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SocketError")
    }
}

fn get_socket(args: &ArgMatches) -> Result<UnixListener, Box<SocketError>> {
    let arg = args.value_of(SOCKET).unwrap();
    let path = Path::new(arg);
    UnixListener::bind(path).map_err(|e| SocketError::new(&e))
}

use std::fs;
use std::io::Read;
use std::{error::Error, os::unix::net::UnixListener, path::Path, path::PathBuf, process, thread};
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
struct SocketError {
    filepath: PathBuf,
    err: String,
}

impl SocketError {
    fn new(path: PathBuf, _e: &::std::io::Error) -> Box<Self> {
        Box::new(SocketError {
            filepath: path.clone(),
            err: _e.to_string(),
        })
    }
}

impl Error for SocketError {}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unable to create socket '{}' due following error: {}",
            self.filepath.to_str().unwrap(),
            self.err
        )
    }
}

fn create_socket(path: PathBuf) -> Result<UnixListener, Box<SocketError>> {
    UnixListener::bind(&path).map_err(|e| SocketError::new(path, &e))
}

fn get_socket_path(args: &ArgMatches) -> PathBuf {
    let arg = args.value_of(SOCKET).unwrap();
    Path::new(arg).to_path_buf()
}

fn get_socket(args: &ArgMatches) -> Result<UnixListener, Box<SocketError>> {
    let path = get_socket_path(args);
    if path.exists() {
        fs::remove_file(&path);
    }
    create_socket(path)
}

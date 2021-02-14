use clap::{App, Arg};

pub static SOCKET: &str = "SOCKET";

pub fn create_app() -> App<'static, 'static> {
    App::new("Timetracker core")
        .version("1.0")
        .author("Samuli S. <samuli.suortti@gmail.com>")
        .about("")
        .arg(Arg::with_name(SOCKET).required(true))
}

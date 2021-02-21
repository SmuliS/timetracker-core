use dbus::blocking::{LocalConnection, Proxy};
use dbus::Error;
use std::time::Duration;

const NAME: &str = "fi.smuli.timetracker";
const PATH: &str = "/fi/smuli/timetracker/status";
const INTERFACE: &str = "fi.smuli.timetracker.status";
const TIMEOUT: Duration = Duration::from_secs(1);
const SET_STATUS_METHOD_NAME: &str = "set";

pub fn update_status(val: &str) -> Result<(), Error> {
    let connection = LocalConnection::new_session()?;
    let proxy = Proxy::new(NAME, PATH, TIMEOUT, &connection);
    let args = (String::from(val),);
    proxy
        .method_call(INTERFACE, SET_STATUS_METHOD_NAME, args)
        .map(|_o: (String,)| ())
}

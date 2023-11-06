use std::sync::mpsc;

// Structure to hold our command and also a sender to siginal when the result has come back
pub struct DBcommand {
    tx: mpsc::Sender<String>,
    cmd: String,
}

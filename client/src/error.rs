pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// prints msg to standard error and exits out of process with given code 
pub fn fail(msg: &str, code: i32) {
    eprintln!("FAILED: {msg}");
    std::process::exit(code);
}

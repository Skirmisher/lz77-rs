use failure::Fail;
use std::io;

#[derive(Debug, Fail)]
pub enum ErrorKind {
	#[fail(display = "io::Error: {}", err)]
	IoError { err: io::Error },
	#[fail(display = "{}", msg)]
	CodeWordError { msg: String },
}

impl From<io::Error> for ErrorKind {
	fn from(err: io::Error) -> Self {
		ErrorKind::IoError { err }
	}
}

impl From<&str> for ErrorKind {
    fn from(msg: &str) -> Self {
        ErrorKind::CodeWordError { msg: msg.to_string() }
    }
}

pub type Result<T> = std::result::Result<T, ErrorKind>;

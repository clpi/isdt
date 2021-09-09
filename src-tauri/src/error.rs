use std::{error, fmt, io, };

pub type IsdtResult<T> = Result<T, IsdtError>;

#[derive(Debug)]
pub enum IsdtError {

}

impl fmt::Display for IsdtError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("")
    }
}

impl error::Error for IsdtError {

}


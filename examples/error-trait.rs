use std::error;
use std::fmt;
use std::io;
use std::net;
use std::fs::File;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError {}

impl From<io::Error> for UpstreamError {
    fn from(err: io::Error) -> UpstreamError {
        UpstreamError::IO(err)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(err: net::AddrParseError) -> UpstreamError {
        UpstreamError::Parsing(err)
    }
}


fn main () -> Result<(),UpstreamError>{

    let _f = File::open("invisible.txt")?;

    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}

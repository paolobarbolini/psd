use crate::PsdChannelKind;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Header(HeaderError),
    Channel(ChannelError),
    Depth(DepthError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Header(ref header) => header.description(),
            Error::Channel(ref channel) => channel.description(),
            Error::Depth(ref depth) => depth.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", (self as &dyn error::Error).description())
    }
}

impl From<HeaderError> for Error {
    fn from(err: HeaderError) -> Error {
        Error::Header(err)
    }
}

impl From<ChannelError> for Error {
    fn from(err: ChannelError) -> Error {
        Error::Channel(err)
    }
}

impl From<DepthError> for Error {
    fn from(err: DepthError) -> Error {
        Error::Depth(err)
    }
}

#[derive(Debug)]
pub enum HeaderError {
    InvalidFileError,
    InvalidSignature,
    InvalidVersion,
    InvalidReserved,
    HeightOutOfRange(u32),
    WidthOutOfRange(u32),
    InvalidColorMode(u8),
}

impl error::Error for HeaderError {
    fn description(&self) -> &str {
        match *self {
            HeaderError::InvalidFileError => "invalid channel type",
            HeaderError::InvalidSignature => "invalid channel type",
            HeaderError::InvalidVersion => "invalid channel type",
            HeaderError::InvalidReserved => "invalid channel type",
            HeaderError::HeightOutOfRange(_) => "invalid channel type",
            HeaderError::WidthOutOfRange(_) => "invalid channel type",
            HeaderError::InvalidColorMode(_) => "invalid channel type",
        }
    }
}

impl fmt::Display for HeaderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", (self as &dyn error::Error).description())
    }
}

#[derive(Debug)]
pub enum ChannelError {
    InvalidId(i16),
    InvalidKind(PsdChannelKind),
    InvalidCompression(u16),
    NotFound(PsdChannelKind),
    OutOfRange(u8),
}

impl error::Error for ChannelError {
    fn description(&self) -> &str {
        match *self {
            ChannelError::InvalidId(_) => "invalid channel type",
            ChannelError::InvalidKind(_) => "invalid channel type",
            ChannelError::InvalidCompression(_) => "limits are exceeded",
            ChannelError::NotFound(_) => "invalid channel type",
            ChannelError::OutOfRange(_) => "invalid channel type",
        }
    }
}

impl fmt::Display for ChannelError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", (self as &dyn error::Error).description())
    }
}

#[derive(Debug)]
pub enum DepthError {
    InvalidDepth(u8),
    UnsupportedDepth,
}

impl error::Error for DepthError {
    fn description(&self) -> &str {
        match *self {
            DepthError::InvalidDepth(_) => "limits are exceeded",
            DepthError::UnsupportedDepth => "limits are exceeded",
        }
    }
}

impl fmt::Display for DepthError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", (self as &dyn error::Error).description())
    }
}

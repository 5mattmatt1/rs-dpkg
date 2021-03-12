#[derive(Debug)]
pub enum Error
{
    IOError(std::io::Error),
    LzmaError(lzma::error::LzmaError),
    Utf8Error(std::str::Utf8Error),
    ParseIntError(std::num::ParseIntError),
    MD5Error,
    InvalidFoldedValue,
    InvalidStatusEntry,
    MissingStatusEntryField(String),
    InvalidPath,
    // Should maybe make this the actual dependency type?
    UnmetDepedency,
}

impl From<std::io::Error> for Error
{
    fn from(error: std::io::Error) -> Self
    {
        Error::IOError(error)
    }
}

impl From<lzma::error::LzmaError> for Error
{
    fn from(error: lzma::error::LzmaError) -> Self
    {
        Error::LzmaError(error)
    }
}

impl From<std::str::Utf8Error> for Error
{
    fn from(error: std::str::Utf8Error) -> Self
    {
        Error::Utf8Error(error)
    }
}

impl From<std::num::ParseIntError> for Error
{
    fn from(error: std::num::ParseIntError) -> Self
    {
        Error::ParseIntError(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>; 

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError(io_error) => {
                io_error.fmt(formatter)
            },
            Error::LzmaError(lzma_error) => {
                lzma_error.fmt(formatter)
            },
            Error::Utf8Error(utf8_error) => {
                utf8_error.fmt(formatter)
            },
            Error::ParseIntError(parse_int_error) => {
                parse_int_error.fmt(formatter)
            },
            Error::MissingStatusEntryField(missing_field) => {
                write!(formatter, "Missing required field \"{}\" from status file.", missing_field)
            },
            Error::MD5Error => {
                formatter.write_str("Invalid String size: MD5 Checksum must be 32 characters long.")
            },
            _ => {
                formatter.write_str(&self.to_string())
            }
        }
    }
}

impl std::error::Error for Error {
    // Only allowing due to function being deprecation in and of itself.
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match self {
            Error::IOError(io_error) => {
                io_error.description()
            },
            Error::LzmaError(lzma_error) => {
                lzma_error.description()
            },
            Error::Utf8Error(utf8_error) => {
                utf8_error.description()
            },
            Error::ParseIntError(parse_int_error) => {
                parse_int_error.description()
            },
            /* and so forth */
            _ => "Unimplemented message",
        }
    }
}
use ::error::*;
use std::error::Error;
use std::fmt;
use std::io::{self, ErrorKind, Read, Write};
use std::io::Error as IoError;

#[derive(Copy, Clone, Debug)]
pub enum SerialError {
    ReadError,
    WriteError,
}

impl fmt::Display for SerialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   SerialError::ReadError => "",
                   SerialError::WriteError => "",
               })
    }
}

impl Error for SerialError {
    fn description(&self) -> &str {
        "Serial I/O Error"
    }
}

/// Wrapper trait for IO mechanisms that drastically reduces boilerplate
pub trait HalSerialIO {
    /// Read some bytes from a robot IO port
    fn hal_read(&mut self, buf: &mut [u8]) -> HalResult<i32>;
    /// Write some bytes to a robot IO port
    fn hal_write(&mut self, buf: &[u8]) -> HalResult<i32>;
    /// Flush the output; This usually just returns `Ok(())`, as many of the
    /// types of IO
    /// that we use don't buffer.
    fn hal_flush(&mut self) -> HalResult<()> {
        Ok(())
    }
}

impl Read for HalSerialIO {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read_count = self.hal_read(buf).map_err(|err| IoError::new(ErrorKind::Other, err))?;

        if read_count <= -1 {
            Err(IoError::new(ErrorKind::Other, SerialError::ReadError))
        } else {
            Ok(read_count as usize)
        }
    }
}

impl Write for HalSerialIO {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let write_count = self.hal_write(buf).map_err(|err| IoError::new(ErrorKind::Other, err))?;

        if write_count <= -1 {
            Err(IoError::new(ErrorKind::Other, SerialError::WriteError))
        } else {
            Ok(write_count as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.hal_flush().map_err(|err| IoError::new(ErrorKind::Other, err))
    }
}

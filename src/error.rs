use crate::unsafe_bindings;
use core::ffi::c_int;

/// Operation successful
pub(crate) const PLIST_ERROR_SUCCESS: c_int = unsafe_bindings::plist_err_t_PLIST_ERR_SUCCESS;

/// All possible errors that can occur when working with plist data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// One or more of the parameters are invalid.
    InvalidArg,
    /// The plist contains nodes not compatible with the output format.
    Format,
    /// Parsing of the input format failed.
    Parse,
    /// Not enough memory to handle the operation.
    NoMem,
    /// I/O error.
    IO,
    /// Unknown error.
    Unknown,
}

impl From<c_int> for Error {
    fn from(code: c_int) -> Error {
        match code {
            // The *success* variant is intentionally omitted in PlistError,
            // since it's not an error... So we manually check it in functions.
            unsafe_bindings::plist_err_t_PLIST_ERR_SUCCESS => panic!("Bug: success variant"),
            unsafe_bindings::plist_err_t_PLIST_ERR_INVALID_ARG => Error::InvalidArg,
            unsafe_bindings::plist_err_t_PLIST_ERR_FORMAT => Error::Format,
            unsafe_bindings::plist_err_t_PLIST_ERR_PARSE => Error::Parse,
            unsafe_bindings::plist_err_t_PLIST_ERR_NO_MEM => Error::NoMem,
            unsafe_bindings::plist_err_t_PLIST_ERR_IO => Error::IO,
            _ => Error::Unknown,
        }
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(_: std::ffi::NulError) -> Self {
        Error::InvalidArg
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Error::InvalidArg => "One or more of the parameters are invalid",
            Error::Format => "The plist contains nodes not compatible with the output format",
            Error::Parse => "Parsing of the input format failed",
            Error::NoMem => "Not enough memory to handle the operation",
            Error::IO => "I/O error",
            Error::Unknown => "Unknown error",
        })
    }
}

impl std::error::Error for Error {}

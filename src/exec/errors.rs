// SPDX-License-Identifier: LGPL-2.1
use std::alloc::LayoutError;
use std::error::Error;
use std::fmt;

/// Any reason why Jazelle execution might have stopped
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExitCondition {
    /// The opcode handler for the given opcode was invoked
    OpcodeHandler(u8),

    /// The program counter ran out-of-bounds
    PCOutOfBounds,

    /// A null value was dereferenced
    #[allow(dead_code)]
    NullPointerException,

    /// An array was accessed out-of-bounds
    #[allow(dead_code)]
    ArrayIndexOutOfBounds,
}

impl fmt::Display for ExitCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ExitCondition {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Any reason why an executor could not be created
#[derive(Debug, PartialEq)]
pub enum CreationError {
    #[allow(dead_code)]
    NotSupported,
    LayoutError(LayoutError),
}

impl From<LayoutError> for CreationError {
    fn from(e: LayoutError) -> CreationError {
        CreationError::LayoutError(e)
    }
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreationError::NotSupported => write!(f, "Not supported on this CPU"),
            e => write!(f, "{e}"),
        }
    }
}

impl Error for CreationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

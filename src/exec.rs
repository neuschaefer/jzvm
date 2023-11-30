// SPDX-License-Identifier: LGPL-2.1
pub use errors::*;
use memmap2::{Mmap, MmapOptions};
use std::fmt::Debug;
use std::io;

mod arm;
mod emu;
mod errors;

/// Create a new executor that is appropriate to the hardware
#[cfg(target_arch = "arm")]
pub fn new() -> Result<Box<dyn Executor>, CreationError> {
    match arm::ARMExecutor::new() {
        Err(CreationError::NotSupported) => Ok(Box::new(emu::EmulationExecutor::new()?)),
        other => Ok(Box::new(other?)),
    }
}

#[cfg(not(target_arch = "arm"))]
pub fn new() -> Result<Box<dyn Executor>, CreationError> {
    Ok(Box::new(emu::EmulationExecutor::new()?))
}

/// Things that can execute Jazelle code
pub trait Executor: Debug {
    /// Enter Jazelle mode and run until an exit condition occurs.
    /// It is unsafe because we're executing arbitrary code...
    unsafe fn execute(&mut self, ctx: &mut Context, state: &mut State)
        -> Result<(), ExitCondition>;

    /// Get the Jazelle ID register contents, if available
    fn get_id(&self) -> Option<u32>;

    /// Get implementation type
    fn get_imp(&self) -> Implementation {
        self.get_id().into()
    }

    /// Get a nicely formatted implementation string
    fn get_id_string(&self) -> String {
        if let Some(id) = self.get_id() {
            format!("{:?} ({:#08x})", self.get_imp(), id)
        } else {
            format!("{:?}", self.get_imp())
        }
    }
}

/// An execution context: It points to the tables necessary for code execution
pub struct Context<'a> {
    /// Code of the current method
    pub code: &'a [u8],

    /// Operand stack
    pub stack: &'a mut [usize],

    /// Table of local values
    pub locals: &'a mut [usize],
}

/// Execution state: Registers changed during execution
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct State {
    /// Program counter, offset into code
    pub pc: usize,

    /// Stack pointer, index into stack
    pub sp: usize,
}

/// Hardware implementation identifier
#[derive(Debug, Clone, Copy)]
pub enum Implementation {
    /// Emulated on an incompatible CPU
    Emulation,

    /// Unknown but Jazelle-compatible CPU
    Unknown,

    /// Trivial implementation
    Trivial,

    /// ARM926EJ-S, my beloved
    ARM926,

    /// ARM1176, as found in early Raspberry Pis
    ARM1176,
}

impl From<u32> for Implementation {
    fn from(id: u32) -> Implementation {
        match id {
            0x64100004 => Implementation::ARM926,
            0x74100064 => Implementation::ARM1176,
            0x00000000 => Implementation::Trivial,
            _ => Implementation::Unknown,
        }
    }
}

impl From<Option<u32>> for Implementation {
    fn from(maybe_id: Option<u32>) -> Implementation {
        match maybe_id {
            Some(id) => id.into(),
            None => Implementation::Emulation,
        }
    }
}

/// A buffer for holding code in an executable page
pub struct CodeBuf {
    map: Mmap,
}

impl TryFrom<&[u8]> for CodeBuf {
    type Error = io::Error;

    fn try_from(code: &[u8]) -> Result<Self, Self::Error> {
        let mut map = MmapOptions::new().len(code.len()).map_anon()?;

        map[..].clone_from_slice(code);
        let map = map.make_exec()?;

        Ok(Self { map })
    }
}

impl AsRef<[u8]> for CodeBuf {
    fn as_ref(&self) -> &[u8] {
        &self.map[..]
    }
}

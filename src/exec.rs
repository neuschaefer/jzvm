// SPDX-License-Identifier: LGPL-2.1
use std::error::Error;
use std::ffi::c_void;
use std::fmt;

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
}

impl From<u32> for Implementation {
    fn from(id: u32) -> Implementation {
        match id {
            0x64100004 => Implementation::ARM926,
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

/// Things that can execute Jazelle code
pub trait Processor {
    /// Enter Jazelle mode and run until an exit condition
    fn execute(&self, state: &mut State) -> ExitCondition;

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

/// Any reason why Jazelle execution might have stopped
pub enum ExitCondition {
    Lol,
}

/// Any reason why an executor could not be created
#[derive(Debug)]
pub enum CreationError {
    NotSupported,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreationError::NotSupported => write!(f, "Not supported on this CPU"),
        }
    }
}

impl Error for CreationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

type HandlerTable = [u32; 0x105]; // TODO: move aligned allocation into
                                  // HandlerTable impl
type Stack = [u32; 1024];

// TODO: Change to a Ctx struct that holds tables by reference instead of owning
// them
pub struct State {
    pub handler_table: Box<HandlerTable>,
    pub stack: Box<Stack>,
    pub stack_pointer: *mut c_void,
}

use std::alloc::LayoutError;
impl State {
    pub fn new() -> Result<State, LayoutError> {
        let handler_table: Box<HandlerTable>;

        unsafe {
            use std::alloc::{alloc_zeroed, Layout};

            let layout = Layout::new::<HandlerTable>().align_to(1024)?;
            handler_table = Box::from_raw(alloc_zeroed(layout) as *mut HandlerTable);
        }

        let mut stack: Box<Stack> = Box::new([0; 1024]);
        let stack_pointer = stack.as_mut_ptr() as *mut c_void;
        Ok(Self {
            handler_table,
            stack,
            stack_pointer,
        })
    }
}

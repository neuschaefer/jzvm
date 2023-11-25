// SPDX-License-Identifier: LGPL-2.1
#![cfg(target_arch = "arm")]

// TODO: move this module into the exec module

use crate::exec::{CreationError, ExitCondition, Processor, State};
use std::arch::asm;

/// Register access.
///
/// Special thanks to the Hackspire project: https://hackspire.org/index.php/Jazelle
mod regs {
    use std::arch::asm;

    /// Read the Jazelle Identity register
    pub unsafe fn read_id() -> u32 {
        let mut id: u32;
        asm!("mrc p14, 7, {reg}, c0, c0, 0", reg = out(reg) id);
        id
    }

    /// Write the Operating System Control register
    ///
    /// This register can only be accessed from privileged modes; these
    /// instructions are UNDEFINED when executed in User mode. EJVMs will
    /// normally never access the Jazelle OS Control register, and EJVMs that
    /// are intended to run in User mode cannot do so. [DDI0100I]
    pub unsafe fn write_os_ctrl(os_ctrl: u32) {
        asm!("mcr p14, 7, {reg}, c1, c0, 0", reg = in(reg) os_ctrl);
    }

    /// Read the Main Configuration register
    ///
    /// This register is normally write-only from User mode. [DDI0100I]
    pub unsafe fn write_main_cfg(cfg: u32) {
        asm!("mcr p14, 7, {reg}, c2, c0, 0", reg = in(reg) cfg);
    }

    /// Read the Array Configuration (object layout) register
    pub unsafe fn read_array_cfg() -> u32 {
        let mut os_ctrl: u32;
        asm!("mrc p14, 7, {reg}, c3, c0, 0", reg = out(reg) os_ctrl);
        os_ctrl
    }
}

/// Hardware-based implementation of the Runner
pub struct ARMProcessor {
    id: u32,
}

impl ARMProcessor {
    pub fn new() -> Result<Self, CreationError> {
        let id: u32;

        unsafe {
            // TODO: install SIGILL handler first
            id = regs::read_id();
        }

        Ok(Self { id })
    }

    pub fn check(&self) {
        unsafe {
            println!("ID:         {:08x}", self.id);
            //println!("Array Cfg:  {:08x}", regs::read_array_cfg());

            // Set Configuration Valid bit
            //regs::write_os_ctrl(2);

            // Set Jazelle Enable bit
            regs::write_main_cfg(1);
        }
        println!("Still alive!");
    }
}

impl Processor for ARMProcessor {
    fn execute(&self, state: &mut State) -> ExitCondition {
        self.check();

        println!("[execute] handlers @ {:?}", state.handler_table.as_ptr());
        println!("[execute] stack    @ {:?}", state.stack.as_ptr());
        println!("[execute] stackptr @ {:?}", state.stack_pointer);

        for i in 0..state.handler_table.len() {
            // TODO: implement a proper handler exit path
            state.handler_table[i] = 0x0dead000 | (i as u32) << 4;
        }

        unsafe {
            asm!(
                // TODO: save ARM register state, Jazelle will clobber a lot
                //
                "mov r5, {handler_table}",
                "mov r6, {stack_pointer}",
                "mov r7, {locals}",

                "adr r12, 3f",
                "adr lr, 1f",
                "bxj r12",

                // TODO: supply code externally
                "1:",

                ".byte 0x05", // iconst_2
                ".byte 0x06", // iconst_3
                ".byte 0x60", // iadd
                ".byte 0xac", // ireturn

                "2:", // TODO: remove
                "b 2b",

                "3:",
                "b 3b",

                // TODO: restore ARM register state before leaving

                handler_table = in(reg) state.handler_table.as_ptr(),
                stack_pointer = in(reg) state.stack_pointer,
                locals = in(reg) state.stack_pointer.add(0x100),
            );
        }

        // Currently:
        //
        // - Jazelle state is entered successfully
        // - Java instructions are executed until ireturn
        // - The ireturn handler is called, resulting in a segfault at 0x0deadac0
        // - The computed value is actually on the stack!
        //   (gdb) p *state.stack
        //   $1 = [5, 0 <repeats 1023 times>]

        ExitCondition::Lol
    }

    fn get_id(&self) -> Option<u32> {
        Some(self.id)
    }
}

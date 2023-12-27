// SPDX-License-Identifier: LGPL-2.1
#![cfg(target_arch = "arm")]

// TODO: move this module into the exec module

use crate::exec::{Context, CreationError, Executor, ExitCondition, State};
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
    pub unsafe fn _write_os_ctrl(os_ctrl: u32) {
        asm!("mcr p14, 7, {reg}, c1, c0, 0", reg = in(reg) os_ctrl);
    }

    /// Read the Main Configuration register
    ///
    /// This register is normally write-only from User mode. [DDI0100I]
    pub unsafe fn write_main_cfg(cfg: u32) {
        asm!("mcr p14, 7, {reg}, c2, c0, 0", reg = in(reg) cfg);
    }

    /// Read the Array Configuration (object layout) register
    pub unsafe fn _read_array_cfg() -> u32 {
        let mut os_ctrl: u32;
        asm!("mrc p14, 7, {reg}, c3, c0, 0", reg = out(reg) os_ctrl);
        os_ctrl
    }
}

#[derive(Debug)]
//struct HandlerTable([u32; 0x105]);
struct HandlerTable([u32; 0x200]);

use std::alloc::LayoutError;
impl HandlerTable {
    fn new() -> Result<Box<Self>, LayoutError> {
        unsafe {
            use std::alloc::{alloc_zeroed, Layout};

            let layout = Layout::new::<Self>().align_to(1024)?;
            Ok(Box::from_raw(alloc_zeroed(layout) as *mut Self))
        }
    }
}

/// Hardware-based implementation of the Executor
#[derive(Debug)]
pub struct ARMExecutor {
    id: u32,
    handler_table: Box<HandlerTable>,
}

impl ARMExecutor {
    pub fn new() -> Result<Self, CreationError> {
        let id: u32;

        unsafe {
            // TODO: install SIGILL handler first
            id = regs::read_id();
        }

        Ok(Self {
            id,
            handler_table: HandlerTable::new()?,
        })
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

impl Executor for ARMExecutor {
    #[allow(named_asm_labels)]
    unsafe fn execute(
        &mut self,
        ctx: &mut Context,
        state: &mut State,
    ) -> Result<(), ExitCondition> {
        self.check();

        for i in 0..self.handler_table.0.len() {
            self.handler_table.0[i] = 0x0dead000 | (i as u32) << 4;
        }

        // Jazelle mode will crash when the locals array is empty. I think
        // that's because local 0 is always prefetched, even when it's unused.
        // To deal with this issue, we provide an alternative locals array.
        let mut dummy_locals: [usize; 1] = [42];
        let locals: &mut [usize] = if ctx.locals.is_empty() {
            &mut dummy_locals
        } else {
            &mut ctx.locals
        };

        unsafe {
            let mut pc = ctx.code.as_ptr().add(state.pc);
            let mut sp = ctx.stack.as_ptr().add(state.sp);
            let locals = locals.as_ptr();
            let mut exit_code: u32;
            let handler_table = self.handler_table.0.as_mut_ptr();

            asm!(
                "push {{r3-r8, r12, lr}}",

                "mov r5, {handler_table}",
                "mov r6, r2",
                "mov r7, {locals}",

                "adr r1, 100f",
                "mov r3, #0",
                "1:",
                "str r1, [r5, r3]",
                "add r3, #4",
                "cmp r3, #0x400",
                "ble 1b",

                "adr r1, 400f",
                "str r1, [r5, #0x400]",
                "adr r1, 404f",
                "str r1, [r5, #0x404]",
                "adr r1, 409f",
                "str r1, [r5, #0x40c]",
                "adr r1, 410f",
                "str r1, [r5, #0x410]",
                "adr r1, 414f",
                "str r1, [r5, #0x414]",

                "adr r12, 900f",
                "mov lr, r0",

                "mov r0, #0",
                "mov r1, #0",
                "mov r2, #0",
                "mov r3, #0",
                "mov r4, #0",

                "bxj r12",

                "fooo:",
                "100:", // Opcode handler
                "mov r1, #0x100",
                "ldrb r0, [lr]",
                "orr r1, r0",
                "b 999f",

                "400:", // Null pointer exception
                "mov r1, #0x00",
                "b 999f",

                "404:", // Array index out of bounds exception
                "mov r1, #0x04",
                "b 999f",

                "409:", // Jazelle disabled
                "mov r1, #0x0c",
                "b 999f",

                "410:", // Configuration invalid
                "mov r1, #0x10",
                "b 999f",

                "414:", // Prefetch abort
                "mov r1, #0x14",
                "b 999f",

                "900:", // Jazelle unsupported
                "mov r1, #0xff",
                "b 999f",

                "999:", // exit
                "mov r0, lr",
                "mov r2, r6",

                "pop {{r3-r8, r12, lr}}",

                // inputs
                handler_table = in(reg) handler_table,
                locals = in(reg) locals,
                inout("r2") sp,
                inout("r0") pc,

                // outputs
                out("r1") exit_code,
            );

            state.pc = pc.offset_from(ctx.code.as_ptr()) as usize;
            state.sp = sp.offset_from(ctx.stack.as_ptr()) as usize;
            println!(
                "exit code: {exit_code:x}, pc: {pc:?} ({}), sp: {sp:?} ({})",
                state.pc, state.sp
            );

            match exit_code {
                0x00 => Err(ExitCondition::NullPointerException),
                0x04 => Err(ExitCondition::ArrayIndexOutOfBounds),
                x if (0x100..0x200).contains(&x) => {
                    Err(ExitCondition::OpcodeHandler((x & 0xff) as u8))
                }
                x => unreachable!("Jazelle exit status {x:x}"),
            }
        }
    }

    fn get_id(&self) -> Option<u32> {
        Some(self.id)
    }
}

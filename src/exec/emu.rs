// SPDX-License-Identifier: LGPL-2.1
use crate::exec::{Context, CreationError, Executor, ExitCondition, State};
use crate::op;

#[derive(Debug, Clone)]
pub struct EmulationExecutor {}

impl EmulationExecutor {
    pub fn new() -> Result<Self, CreationError> {
        Ok(EmulationExecutor {})
    }
}

fn fetch_u8(ctx: &Context, pc: usize) -> Result<u8, ExitCondition> {
    let ptr = ctx.code.get(pc).ok_or(ExitCondition::PCOutOfBounds)?;
    Ok(*ptr)
}

fn i2us(i: i32) -> usize {
    i as u32 as usize
}

impl Executor for EmulationExecutor {
    unsafe fn execute(
        &mut self,
        ctx: &mut Context,
        state: &mut State,
    ) -> Result<(), ExitCondition> {
        loop {
            match fetch_u8(ctx, state.pc)? {
                op::nop => {}
                op::aconst_null => {
                    ctx.stack[state.sp] = 0; //JReference::null().into(),
                    state.sp += 1;
                }
                op::iconst_m1 => {
                    ctx.stack[state.sp] = i2us(-1);
                    state.sp += 1;
                }
                op::iconst_2 => {
                    ctx.stack[state.sp] = 2;
                    state.sp += 1;
                }
                op::iconst_3 => {
                    ctx.stack[state.sp] = 3;
                    state.sp += 1;
                }
                op::istore_0 => {
                    state.sp -= 1;
                    let x = ctx.stack[state.sp];
                    ctx.locals[0] = x;
                }
                op::iadd => {
                    state.sp -= 2;
                    let a = ctx.stack[state.sp];
                    let b = ctx.stack[state.sp + 1];
                    ctx.stack[state.sp] = a + b;
                    state.sp += 1;
                }
                opcode => return Err(ExitCondition::OpcodeHandler(opcode)),
            }

            state.pc += 1;
        }
    }

    fn get_id(&self) -> Option<u32> {
        None
    }
}

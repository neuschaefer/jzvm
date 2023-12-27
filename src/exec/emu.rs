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

fn l2us(l: isize) -> usize {
    l as usize
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
                op::iconst_0 => {
                    ctx.stack[state.sp] = i2us(0);
                    state.sp += 1;
                }
                op::iconst_1 => {
                    ctx.stack[state.sp] = i2us(1);
                    state.sp += 1;
                }
                op::iconst_2 => {
                    ctx.stack[state.sp] = i2us(2);
                    state.sp += 1;
                }
                op::iconst_3 => {
                    ctx.stack[state.sp] = i2us(3);
                    state.sp += 1;
                }
                op::iconst_4 => {
                    ctx.stack[state.sp] = i2us(4);
                    state.sp += 1;
                }
                op::iconst_5 => {
                    ctx.stack[state.sp] = i2us(5);
                    state.sp += 1;
                }
                op::lconst_0 => {
                    ctx.stack[state.sp] = 0;
                    ctx.stack[state.sp + 1] = 0; // this is wrong
                    state.sp += 2;
                }
                op::lconst_1 => {
                    ctx.stack[state.sp] = 1;
                    ctx.stack[state.sp + 1] = 0;
                    state.sp += 2;
                }
                op::fconst_0 => {
                    ctx.stack[state.sp] = 0;
                    state.sp += 1;
                }
                op::fconst_1 => {
                    ctx.stack[state.sp] = 0x3f800000; // dirty dirty hack to
                                                      // hardcode it like this...
                    state.sp += 1;
                }
                op::fconst_2 => {
                    ctx.stack[state.sp] = 0x40000000;
                    state.sp += 1;
                }
                op::dconst_0 => {
                    ctx.stack[state.sp] = 0;
                    ctx.stack[state.sp + 1] = 0;
                    state.sp += 2;
                }
                op::bipush => {
                    state.pc += 1;
                    let value = fetch_u8(ctx, state.pc)?; // here, likewise, add a
                                                          // function to get one byte
                    ctx.stack[state.sp] = value as i8 as i32 as u32 as usize;
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

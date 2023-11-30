// SPDX-License-Identifier: LGPL-2.1
use std::error::Error;
mod exec;
mod op;

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = exec::State::default();
    let mut exec = exec::new()?;

    let code: exec::CodeBuf =
        [op::iconst_2, op::iconst_3, op::iadd, op::breakpoint][..].try_into()?;
    let mut ctx = exec::Context {
        code: code.as_ref(),
        locals: &mut vec![0],
        stack: &mut vec![0; 2],
    };

    println!("Jazelle implementation: {}", exec.get_id_string());

    unsafe {
        println!("exec -> {:?}", exec.execute(&mut ctx, &mut state));
    }

    println!("Stack: {:x?}", &ctx.stack[0..state.sp]);

    Ok(())
}

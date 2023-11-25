// SPDX-License-Identifier: LGPL-2.1
use crate::exec::*;
use std::error::Error;

mod arm;
pub mod exec;

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::new()?;
    let proc = arm::ARMProcessor::new()?;

    println!("Jazelle implementation: {}", proc.get_id_string());
    proc.check();

    proc.execute(&mut state);

    Ok(())
}

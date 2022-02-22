use revm::Return;

use crate::machine::machine::Machine;

#[inline(always)]
pub fn add(machine: &mut Machine) -> Return {
    pop_top!(machine, op1, op2);
    let (ret, ..) = op1.overflowing_add(*op2);
    *op2 = ret;

    Return::Continue
}

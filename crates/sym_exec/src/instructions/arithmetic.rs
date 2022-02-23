use revm::Return;

use crate::machine::machine::Machine;
use crate::machine::stack::Stack;
use crate::sym::{SymWord, IR};

#[inline(always)]
pub fn add<IStack: Stack>(machine: &mut Machine<IStack>) -> Return {
    let op1 = machine.stack.pop();

    let op1 = match op1 {
        Ok(x) => x,
        Err(ret) => return ret,
    };

    let op2 = machine.stack.pop();

    let op2 = match op2 {
        Ok(x) => x,
        Err(ret) => return ret,
    };

    let ret = match (op1, op2) {
        (SymWord::Sym(op1), SymWord::Sym(op2)) => {
            IR::ADD(op1.into(), op2.into()).into()
        }

        (SymWord::Sym(op1), SymWord::Concrete(op2)) => {
            IR::ADD(op1.into(), IR::Base(op2).into()).into()
        }

        (SymWord::Concrete(op1), SymWord::Sym(op2)) => {
            IR::ADD(IR::Base(op1).into(), op2.into()).into()
        }

        (SymWord::Concrete(op1), SymWord::Concrete(op2)) => {
            op1.overflowing_add(op2).0.into()
        }
    };

    match machine.stack.push(ret) {
        Ok(()) => {}
        Err(ret) => return ret,
    };

    Return::Continue
}

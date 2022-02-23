use revm::{Host, Return, Spec};
use crate::{instructions::eval, sym::IR};

use super::stack::Stack;

pub struct Machine<IStack: Stack> {
    pub program_counter: *const u8,
    pub stack: IStack,
    pub constraints: Vec<IR>
}

impl <IStack: Stack> Machine<IStack> {
    /// See comments in `crates/revm/src/machine/machine.rs`
    pub fn run<H: Host, SPEC: Spec>(&mut self, host: &mut H) -> Return {
        let mut ret = Return::Continue;

        while ret == Return::Continue {
            let opcode = unsafe { *self.program_counter };
            self.program_counter = unsafe { self.program_counter.offset(1) };
            ret = eval::<H, SPEC, IStack>(opcode, self, host);
        }

        ret
    }
}

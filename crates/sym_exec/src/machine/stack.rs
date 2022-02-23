use revm::Return;

use crate::sym::SymWord;

pub trait Stack {
    fn len(&self) -> usize;
    fn pop(&mut self) -> Result<SymWord, Return>;
    fn push(&mut self, val: SymWord) -> Result<(), Return>;
}

struct VecBackedSymStack {
    data: Vec<SymWord>
}

// TODO import from
// use revm::machine::stack::STACK_LIMIT;
pub const STACK_LIMIT: usize = 1024;

impl VecBackedSymStack {
    /// Create a new stack with given limit.
    pub fn new() -> Self {
        Self {
            // Safety: A lot of functions assumes that capacity is STACK_LIMIT
            data: Vec::with_capacity(STACK_LIMIT),
        }
    }
}

impl Stack for VecBackedSymStack {
    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn pop(&mut self) -> Result<SymWord, Return> {
        self.data.pop().ok_or(Return::StackUnderflow)
    }

    #[inline]
    fn push(&mut self, value: SymWord) -> Result<(), Return> {
        if self.data.len() + 1 > STACK_LIMIT {
            return Err(Return::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }
}

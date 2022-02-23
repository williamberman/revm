mod z3;

use primitive_types::U256;

pub enum SymWord {
    Sym(IR),
    Concrete(U256)
}

impl From<IR> for SymWord {
    fn from(x: IR) -> Self {
        SymWord::Sym(x)
    }
}

impl From<U256> for SymWord {
    fn from(x: U256) -> Self {
        SymWord::Concrete(x)
    }
}

#[derive(Debug)]
pub enum IR {
    ADD(Box<IR>, Box<IR>),
    EqZero(Box<IR>),
    NotEqZero(Box<IR>),
    Base(U256)
}

impl From<U256> for IR {
    fn from(x: U256) -> Self {
        IR::Base(x)
    }
}

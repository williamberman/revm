use std::convert::TryInto;

use z3::{ast::{self, Ast}, SatResult};

use super::IR;

static WORD_SIZE_BITS: u32 = 256;

impl IR {
    fn as_constraint<'ctx>(&self, ctx: &'ctx z3::Context) -> ast::Bool<'ctx> {
        let zero = ast::BV::from_u64(ctx, 0, WORD_SIZE_BITS);

        match self {
            IR::EqZero(x) => x.as_bv(ctx)._eq(&zero),
            IR::NotEqZero(x) => x.as_bv(ctx)._eq(&zero).not(),
            _ => panic!("cannot make constraint out of term: {:?}", self),
        }
    }

    fn as_bv<'ctx>(&self, ctx: &'ctx z3::Context) -> ast::BV<'ctx> {
        match self {
            IR::ADD(x, y) => x.as_bv(ctx).bvadd(&y.as_bv(ctx)),
            IR::Base(x) => ast::BV::from_u64(ctx, x.as_u64(), WORD_SIZE_BITS),
            _ => panic!("cannot make bitvector out of term: {:?}", self),
        }
    }
}

pub fn solve<'a, Iter>(constraints: Iter, calldata_length: u64) -> Option<Vec<u8>>
where
    Iter: Iterator<Item = &'a IR>,
{
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let calldata = z3::FuncDecl::new(
        &ctx,
        "calldata",
        &[&z3::Sort::bitvector(&ctx, WORD_SIZE_BITS)],
        &z3::Sort::bitvector(&ctx, 8),
    );

    constraints.for_each(|x| solver.assert(&x.as_constraint(&ctx)));

    let res = solver.check();

    if SatResult::Sat != res {
        return None
    };

    let model = solver.get_model().unwrap();

    let mut rv = Vec::with_capacity(calldata_length.try_into().unwrap());

    for i in 0..calldata_length {
        let sym_byte= calldata.apply(&[&ast::BV::from_u64(&ctx, i, WORD_SIZE_BITS)]);
        let xcalldata = model.eval(&sym_byte, true).unwrap();
        let byte = xcalldata.as_bv().unwrap().as_u64().unwrap();
        assert!(byte <= 255);
        let byte = byte.to_le_bytes()[0];
        rv.push(byte);
    }

    Some(rv)
}

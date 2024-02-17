use core::fmt;
use p3_field::{AbstractField, PrimeField64};
use std::{
    borrow::Borrow,
    hash::Hash,
    iter::{Product, Sum},
    marker::PhantomData,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use z3::{ast::*, *};
use z3_sys::*;

use crate::context::context;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Felt<'ctx, F: PrimeField64>(Int<'ctx>, PhantomData<F>);

impl<'ctx, F: PrimeField64> Felt<'ctx, F> {
    pub fn from_int(x: Int<'ctx>) -> Self {
        Self(x, PhantomData::<F>)
    }

    pub fn new_const<S: Into<Symbol>>(solver: &'ctx Solver, name: S) -> Self {
        let ctx = solver.get_context();
        let x = Self::from_int(Int::new_const(ctx, name));

        let zero = Int::from_u64(&ctx, 0);
        let p = Int::from_u64(&ctx, F::ORDER_U64);
        solver.assert(&x.0.ge(&zero));
        solver.assert(&x.0.lt(&p));

        x
    }

    pub fn from_u64(ctx: &'ctx Context, u: u64) -> Self {
        assert!(u < F::ORDER_U64);
        Self::from_int(Int::from_u64(ctx, u))
    }

    pub fn from_f(ctx: &'ctx Context, u: F) -> Self {
        Self::from_int(Int::from_u64(ctx, u.as_canonical_u64()))
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.0.as_u64()
    }

    pub fn add(ctx: &'ctx Context, values: &[impl Borrow<Self>]) -> Self {
        let tmp = Int::add(
            ctx,
            values
                .iter()
                .map(|x| &x.borrow().0)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        Self::from_int(tmp.modulo(&Int::from_u64(ctx, F::ORDER_U64)))
    }

    pub fn sub(ctx: &'ctx Context, values: &[impl Borrow<Self>]) -> Self {
        let tmp = Int::sub(
            ctx,
            values
                .iter()
                .map(|x| &x.borrow().0)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        Self::from_int(tmp.modulo(&Int::from_u64(ctx, F::ORDER_U64)))
    }

    pub fn mul(ctx: &'ctx Context, values: &[impl Borrow<Self>]) -> Self {
        let tmp = Int::mul(
            ctx,
            values
                .iter()
                .map(|x| &x.borrow().0)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        Self::from_int(tmp.modulo(&Int::from_u64(ctx, F::ORDER_U64)))
    }

    pub fn unary_minus(&self) -> Self {
        let tmp = self.0.unary_minus();
        Self::from_int(tmp.modulo(&Int::from_u64(self.get_ctx(), F::ORDER_U64)))
    }
}

impl<'ctx, F: PrimeField64> Ast<'ctx> for Felt<'ctx, F> {
    unsafe fn wrap(ctx: &'ctx Context, ast: Z3_ast) -> Self {
        Self::from_int(Int::wrap(ctx, ast))
    }

    fn get_ctx(&self) -> &'ctx Context {
        self.0.get_ctx()
    }

    fn get_z3_ast(&self) -> Z3_ast {
        self.0.get_z3_ast()
    }
}

impl<'ctx, F: PrimeField64> fmt::Debug for Felt<'ctx, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl<'ctx, F: PrimeField64> fmt::Display for Felt<'ctx, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl<'ctx, F: PrimeField64> Default for Felt<'ctx, F> {
    fn default() -> Self {
        Self::from_u64(context(), 0)
    }
}

impl<'ctx, F: PrimeField64> From<F> for Felt<'ctx, F> {
    fn from(value: F) -> Self {
        Self::from_f(context(), value)
    }
}

impl<'ctx, F: PrimeField64> Add<Felt<'ctx, F>> for Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn add(self, other: Felt<'ctx, F>) -> Self::Output {
        Felt::add(
            self.get_ctx(),
            &[&self as &Felt<'ctx, F>, &other as &Felt<'ctx, F>],
        )
    }
}

impl<'ctx, F: PrimeField64> Add<&Felt<'ctx, F>> for &Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn add(self, other: &Felt<'ctx, F>) -> Self::Output {
        Felt::add(
            self.get_ctx(),
            &[&self as &Felt<'ctx, F>, &other as &Felt<'ctx, F>],
        )
    }
}

impl<'ctx, F: PrimeField64> AddAssign<Felt<'ctx, F>> for Felt<'ctx, F> {
    fn add_assign(&mut self, other: Felt<'ctx, F>) {
        *self = Felt::add(
            self.get_ctx(),
            &[&self as &Felt<'ctx, F>, &other as &Felt<'ctx, F>],
        )
    }
}

impl<'ctx, F: PrimeField64> Add<F> for Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn add(self, other: F) -> Self::Output {
        let ctx = self.get_ctx();
        Felt::add(ctx, &[&self as &Felt<'ctx, F>, &Self::from_f(ctx, other)])
    }
}

impl<'ctx, F: PrimeField64> Sum for Felt<'ctx, F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y).unwrap_or(Self::zero())
    }
}

impl<'ctx, F: PrimeField64> Sub<Felt<'ctx, F>> for Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn sub(self, other: Felt<'ctx, F>) -> Self::Output {
        Felt::sub(
            self.get_ctx(),
            &[&self as &Felt<'ctx, F>, &other as &Felt<'ctx, F>],
        )
    }
}

impl<'ctx, F: PrimeField64> Sub<&Felt<'ctx, F>> for &Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn sub(self, other: &Felt<'ctx, F>) -> Self::Output {
        Felt::sub(
            self.get_ctx(),
            &[&self as &Felt<'ctx, F>, &other as &Felt<'ctx, F>],
        )
    }
}

impl<'ctx, F: PrimeField64> SubAssign<Felt<'ctx, F>> for Felt<'ctx, F> {
    fn sub_assign(&mut self, other: Felt<'ctx, F>) {
        *self = Felt::sub(
            self.get_ctx(),
            &[&self as &Felt<'ctx, F>, &other as &Felt<'ctx, F>],
        )
    }
}

impl<'ctx, F: PrimeField64> Sub<F> for Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn sub(self, other: F) -> Self::Output {
        let ctx = self.get_ctx();
        Felt::sub(ctx, &[&self as &Felt<'ctx, F>, &Self::from_f(ctx, other)])
    }
}

impl<'ctx, F: PrimeField64> Mul<Felt<'ctx, F>> for Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn mul(self, other: Felt<'ctx, F>) -> Self::Output {
        Felt::mul(
            self.get_ctx(),
            &[&self as &Felt<'ctx, F>, &other as &Felt<'ctx, F>],
        )
    }
}

impl<'ctx, F: PrimeField64> Mul<&Felt<'ctx, F>> for &Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn mul(self, other: &Felt<'ctx, F>) -> Self::Output {
        Felt::mul(
            self.get_ctx(),
            &[&self as &Felt<'ctx, F>, &other as &Felt<'ctx, F>],
        )
    }
}

impl<'ctx, F: PrimeField64> MulAssign<Felt<'ctx, F>> for Felt<'ctx, F> {
    fn mul_assign(&mut self, other: Felt<'ctx, F>) {
        *self = Felt::mul(
            self.get_ctx(),
            &[&self as &Felt<'ctx, F>, &other as &Felt<'ctx, F>],
        )
    }
}

impl<'ctx, F: PrimeField64> Mul<F> for Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn mul(self, other: F) -> Self::Output {
        let ctx = self.get_ctx();
        Felt::mul(ctx, &[&self as &Felt<'ctx, F>, &Self::from_f(ctx, other)])
    }
}

impl<'ctx, F: PrimeField64> Product for Felt<'ctx, F> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x * y).unwrap_or(Self::one())
    }
}

impl<'ctx, F: PrimeField64> Neg for Felt<'ctx, F> {
    type Output = Felt<'ctx, F>;

    fn neg(self) -> Self::Output {
        self.unary_minus()
    }
}

impl<'ctx, F: PrimeField64> AbstractField for Felt<'ctx, F> {
    type F = F;

    fn zero() -> Self {
        Self::from_f(F::zero())
    }
    fn one() -> Self {
        Self::from_f(F::one())
    }
    fn two() -> Self {
        Self::from_f(F::two())
    }
    fn neg_one() -> Self {
        Self::from_f(F::neg_one())
    }

    #[inline]
    fn from_f(f: Self::F) -> Self {
        f.into()
    }

    fn from_bool(b: bool) -> Self {
        Self::from_f(F::from_bool(b))
    }

    fn from_canonical_u8(n: u8) -> Self {
        Self::from_f(F::from_canonical_u8(n))
    }

    fn from_canonical_u16(n: u16) -> Self {
        Self::from_f(F::from_canonical_u16(n))
    }

    fn from_canonical_u32(n: u32) -> Self {
        Self::from_f(F::from_canonical_u32(n))
    }

    fn from_canonical_u64(n: u64) -> Self {
        Self::from_f(F::from_canonical_u64(n))
    }

    fn from_canonical_usize(n: usize) -> Self {
        Self::from_f(F::from_canonical_usize(n))
    }

    fn from_wrapped_u32(n: u32) -> Self {
        Self::from_f(F::from_wrapped_u32(n))
    }

    fn from_wrapped_u64(n: u64) -> Self {
        Self::from_f(F::from_wrapped_u64(n))
    }

    fn generator() -> Self {
        Self::from_f(F::generator())
    }
}

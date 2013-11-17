// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[lang="drop"]
pub trait Drop {
    fn drop(&mut self);
}

#[lang="add"]
pub trait Add<RHS, Result> {
    fn add(&self, rhs: &RHS) -> Result;
}

#[lang="sub"]
pub trait Sub<RHS, Result> {
    fn sub(&self, rhs: &RHS) -> Result;
}

#[lang="mul"]
pub trait Mul<RHS, Result> {
    fn mul(&self, rhs: &RHS) -> Result;
}

#[lang="div"]
pub trait Div<RHS, Result> {
    fn div(&self, rhs: &RHS) -> Result;
}

#[lang="rem"]
pub trait Rem<RHS, Result> {
    fn rem(&self, rhs: &RHS) -> Result;
}

#[lang="neg"]
pub trait Neg<Result> {
    fn neg(&self) -> Result;
}

#[lang="not"]
pub trait Not<Result> {
    fn not(&self) -> Result;
}

#[lang="bitand"]
pub trait BitAnd<RHS, Result> {
    fn bitand(&self, rhs: &RHS) -> Result;
}

#[lang="bitor"]
pub trait BitOr<RHS, Result> {
    fn bitor(&self, rhs: &RHS) -> Result;
}

#[lang="bitxor"]
pub trait BitXor<RHS, Result> {
    fn bitxor(&self, rhs: &RHS) -> Result;
}

#[lang="shl"]
pub trait Shl<RHS, Result> {
    fn shl(&self, rhs: &RHS) -> Result;
}

#[lang="shr"]
pub trait Shr<RHS, Result> {
    fn shr(&self, rhs: &RHS) -> Result;
}

#[lang="index"]
pub trait Index<Index, Result> {
    fn index(&self, index: &Index) -> Result;
}

#[lang="eq"]
pub trait Eq {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool { !self.ne(other) }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool { !self.eq(other) }
}

#[lang="ord"]
pub trait Ord {
    #[inline(always)]
    fn lt(&self, other: &Self) -> bool { other.gt(self) }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool { !other.lt(self) }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool { other.lt(self) }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool { !self.lt(other) }
}

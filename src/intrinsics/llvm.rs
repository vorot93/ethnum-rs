//! This module contains definitions for LLVM IR generated intrinsics.

// NOTE: LLVM IR generated intrinsics for `udiv i256` and `urem i256` produce an
// error when compiling, so use the native `divmod` implementation even when
// generated intrinsics are enabled.
#[path = "native/divmod.rs"]
mod divmod;

pub use self::divmod::*;
use crate::{int::I256, uint::U256};
use core::mem::{self, MaybeUninit};

pub fn imulc(_: &mut MaybeUninit<I256>, _: &I256, _: &I256) -> bool {
    todo!()
}

macro_rules! def {
    ($(
        $(#[$a:meta])*
        pub fn $name:ident(
            $($p:ident : $t:ty),*
        ) $(-> $ret:ty)?;
    )*) => {$(
        $(#[$a])*
        pub fn $name(
            $($p: $t,)*
        ) $(-> $ret)? {
            unsafe {
                ethnum_intrinsics::$name($(
                    #[allow(clippy::transmute_ptr_to_ptr)]
                    mem::transmute($p)
                ),*)
            }
        }
    )*};
}

def! {
    pub fn add2(r: &mut U256, a: &U256);
    pub fn add3(r: &mut MaybeUninit<U256>, a: &U256, b: &U256);
    pub fn uaddc(r: &mut MaybeUninit<U256>, a: &U256, b: &U256) -> bool;
    pub fn iaddc(r: &mut MaybeUninit<I256>, a: &I256, b: &I256) -> bool;

    pub fn sub2(r: &mut U256, a: &U256);
    pub fn sub3(r: &mut MaybeUninit<U256>, a: &U256, b: &U256);
    pub fn usubc(r: &mut MaybeUninit<U256>, a: &U256, b: &U256) -> bool;
    pub fn isubc(r: &mut MaybeUninit<I256>, a: &I256, b: &I256) -> bool;

    pub fn umul2(r: &mut U256, a: &U256);
    pub fn umul3(r: &mut MaybeUninit<U256>, a: &U256, b: &U256);
    pub fn umulc(r: &mut MaybeUninit<U256>, a: &U256, b: &U256) -> bool;
    pub fn imul2(r: &mut I256, a: &I256);
    pub fn imul3(r: &mut MaybeUninit<I256>, a: &I256, b: &I256);
    //pub fn imulc(r: &mut MaybeUninit<U256>, a: &U256, b: &U256) -> bool;

    pub fn ashl2(r: &mut U256, a: u32);
    pub fn ashl3(r: &mut MaybeUninit<U256>, a: &U256, b: u32);

    pub fn ashr2(r: &mut I256, a: u32);
    pub fn ashr3(r: &mut MaybeUninit<I256>, a: &I256, b: u32);
    pub fn lshr2(r: &mut U256, a: u32);
    pub fn lshr3(r: &mut MaybeUninit<U256>, a: &U256, b: u32);

    pub fn rotate_left(r: &mut MaybeUninit<U256>, a: &U256, b: u32);
    pub fn rotate_right(r: &mut MaybeUninit<U256>, a: &U256, b: u32);

    pub fn ctlz(a: &U256) -> u32;
    pub fn cttz(a: &U256) -> u32;
}

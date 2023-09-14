#![cfg_attr(feature = "nightly", feature(atomic_mut_ptr))]
use std::sync::atomic::{self, Ordering};
use std::hash::Hash;
use std::fmt::{Debug, Display};
use std::panic::{RefUnwindSafe, UnwindSafe};

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign,
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign,
    Shl, ShlAssign, Shr, ShrAssign,
    Not,
};

pub trait AtomicInt : Default + Send + Sync + RefUnwindSafe + UnwindSafe {
    type Prim
        : Copy + Debug + Display + Eq + Hash + Ord + Sized
        + Add + AddAssign
        + BitAnd + BitAndAssign
        + BitOr + BitOrAssign
        + BitXor + BitXorAssign
        + Div + DivAssign
        + Mul + MulAssign
        + Not
        + Rem + RemAssign
        + Shl + ShlAssign
        + Shr + ShrAssign
        + Sub + SubAssign;

    const ZERO: <Self as AtomicInt>::Prim;
    const ONE: <Self as AtomicInt>::Prim;

    const MIN: <Self as AtomicInt>::Prim;
    const MAX: <Self as AtomicInt>::Prim;

    fn new(val: <Self as AtomicInt>::Prim) -> Self;

    fn fetch_add(
        &self,
        new: <Self as AtomicInt>::Prim,
        ordering: Ordering
    ) -> <Self as AtomicInt>::Prim;

    fn fetch_sub(
        &self,
        new: <Self as AtomicInt>::Prim,
        ordering: Ordering
    ) -> <Self as AtomicInt>::Prim;

    fn fetch_and(
        &self,
        new: <Self as AtomicInt>::Prim,
        ordering: Ordering
    ) -> <Self as AtomicInt>::Prim;

    fn fetch_nand(
        &self,
        new: <Self as AtomicInt>::Prim,
        ordering: Ordering
    ) -> <Self as AtomicInt>::Prim;

    fn fetch_or(
        &self,
        new: <Self as AtomicInt>::Prim,
        ordering: Ordering
    ) -> <Self as AtomicInt>::Prim;

    fn fetch_xor(
        &self,
        new: <Self as AtomicInt>::Prim,
        ordering: Ordering
    ) -> <Self as AtomicInt>::Prim;

    fn fetch_min(&self, val: <Self as AtomicInt>::Prim, order: Ordering) -> <Self as AtomicInt>::Prim;

    fn fetch_max(&self, val: <Self as AtomicInt>::Prim, order: Ordering) -> <Self as AtomicInt>::Prim;

    fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F
    ) -> Result<<Self as AtomicInt>::Prim, <Self as AtomicInt>::Prim>
    where F: FnMut(<Self as AtomicInt>::Prim) -> Option<<Self as AtomicInt>::Prim>;

    fn get_mut(&mut self) -> &mut <Self as AtomicInt>::Prim;

    fn into_inner(self) -> <Self as AtomicInt>::Prim;

    fn load(&self, order: Ordering) -> <Self as AtomicInt>::Prim;

    fn store(&self, val: <Self as AtomicInt>::Prim, order: Ordering);

    fn swap(&self, val: <Self as AtomicInt>::Prim, order: Ordering) -> <Self as AtomicInt>::Prim;

    fn compare_exchange(
        &self,
        current: <Self as AtomicInt>::Prim,
        new: <Self as AtomicInt>::Prim,
        success: Ordering,
        failure: Ordering
    ) -> Result<<Self as AtomicInt>::Prim, <Self as AtomicInt>::Prim>;

    fn compare_exchange_weak(
        &self,
        current: <Self as AtomicInt>::Prim,
        new: <Self as AtomicInt>::Prim,
        success: Ordering,
        failure: Ordering
    ) -> Result<<Self as AtomicInt>::Prim, <Self as AtomicInt>::Prim>;

    #[cfg(feature="nightly")]
    fn as_mut_ptr(&self) -> *mut <Self as AtomicInt>::Prim;
}

macro_rules! impl_atomic_int {
    ($atomic:ty = $prim:ty) => {
        impl AtomicInt for $atomic {
            const ZERO: $prim = 0;
            const ONE: $prim = 1;

            const MIN: $prim = <$prim>::MIN;
            const MAX: $prim = <$prim>::MAX;

            type Prim = $prim;

            fn new(val: $prim) -> Self {
                <$atomic>::new(val)
            }

            fn fetch_add(
                &self,
                new: $prim,
                ordering: Ordering,
            ) -> $prim {
                self.fetch_add(new, ordering)
            }

            fn fetch_sub(
                &self,
                new: $prim,
                ordering: Ordering,
            ) -> $prim {
                self.fetch_sub(new, ordering)
            }

            fn fetch_and(
                &self,
                new: $prim,
                ordering: Ordering,
            ) -> $prim {
                self.fetch_and(new, ordering)
            }

            fn fetch_nand(
                &self,
                new: $prim,
                ordering: Ordering,
            ) -> $prim {
                self.fetch_nand(new, ordering)
            }

            fn fetch_or(
                &self,
                new: $prim,
                ordering: Ordering,
            ) -> $prim {
                self.fetch_or(new, ordering)
            }

            fn fetch_xor(
                &self,
                new: $prim,
                ordering: Ordering,
            ) -> $prim {
                self.fetch_xor(new, ordering)
            }

            fn fetch_min(
                &self,
                val: $prim,
                ordering: Ordering,
            ) -> $prim {
                self.fetch_min(val, ordering)
            }

            fn fetch_max(
                &self,
                val: $prim,
                ordering: Ordering,
            ) -> $prim {
                self.fetch_max(val, ordering)
            }

            fn fetch_update<F>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                f: F,
            ) -> Result<$prim, $prim>
            where
                F: FnMut($prim) -> Option<$prim>,
            {
                self.fetch_update(set_order, fetch_order, f)
            }

            fn get_mut(&mut self) -> &mut $prim {
                self.get_mut()
            }

            fn into_inner(self) -> $prim {
                self.into_inner()
            }

            fn load(&self, order: Ordering) -> $prim {
                self.load(order)
            }

            fn store(&self, val: $prim, order: Ordering) {
                self.store(val, order)
            }

            fn swap(&self, val: $prim, order: Ordering) -> $prim {
                self.swap(val, order)
            }

            fn compare_exchange(
                &self,
                current: $prim,
                new: $prim,
                success: Ordering,
                failure: Ordering
            ) -> Result<$prim, $prim> {
                self.compare_exchange(current, new, success, failure)
            }

            fn compare_exchange_weak(
                &self,
                current: $prim,
                new: $prim,
                success: Ordering,
                failure: Ordering
            ) -> Result<$prim, $prim> {
                self.compare_exchange_weak(current, new, success, failure)
            }

            #[cfg(feature="nightly")]
            fn as_mut_ptr(&self) -> *mut $prim {
                self.as_mut_ptr()
            }
        }
    };
}

impl_atomic_int!(atomic::AtomicU8 = u8);
impl_atomic_int!(atomic::AtomicU16 = u16);
impl_atomic_int!(atomic::AtomicU32 = u32);
impl_atomic_int!(atomic::AtomicU64 = u64);
impl_atomic_int!(atomic::AtomicUsize = usize);

impl_atomic_int!(atomic::AtomicI8 = i8);
impl_atomic_int!(atomic::AtomicI16 = i16);
impl_atomic_int!(atomic::AtomicI32 = i32);
impl_atomic_int!(atomic::AtomicI64 = i64);
impl_atomic_int!(atomic::AtomicIsize = isize);

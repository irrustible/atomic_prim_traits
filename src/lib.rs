#![cfg_attr(feature = "nightly", feature(atomic_min_max, atomic_mut_ptr, no_more_cas))]
use std::sync::atomic::{self, Ordering};
use std::panic::{RefUnwindSafe, UnwindSafe};

pub trait AtomicInt : Default + Send + Sync + RefUnwindSafe + UnwindSafe {
    type Prim: Sized + Eq + Ord;

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

    #[cfg(feature="nightly")]
    fn fetch_min(&self, val: <Self as AtomicInt>::Prim, order: Ordering) -> <Self as AtomicInt>::Prim;

    #[cfg(feature="nightly")]
    fn fetch_max(&self, val: <Self as AtomicInt>::Prim, order: Ordering) -> <Self as AtomicInt>::Prim;

    #[cfg(feature="nightly")]
    fn fetch_update<F>(
        &self,
        f: F,
        fetch_order: Ordering,
        set_order: Ordering
    ) -> Result<<Self as AtomicInt>::Prim, <Self as AtomicInt>::Prim>
    where F: FnMut(<Self as AtomicInt>::Prim) -> Option<<Self as AtomicInt>::Prim>;

    fn get_mut(&mut self) -> &mut <Self as AtomicInt>::Prim;

    fn into_inner(self) -> <Self as AtomicInt>::Prim;

    fn load(&self, order: Ordering) -> <Self as AtomicInt>::Prim;

    fn store(&self, val: <Self as AtomicInt>::Prim, order: Ordering);

    fn swap(&self, val: <Self as AtomicInt>::Prim, order: Ordering) -> <Self as AtomicInt>::Prim;

    fn compare_and_swap(
        &self,
        current: <Self as AtomicInt>::Prim,
        new: <Self as AtomicInt>::Prim,
        ordering: Ordering
    ) -> <Self as AtomicInt>::Prim;

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

impl AtomicInt for atomic::AtomicU8 {
    type Prim = u8;

    fn new(val: u8) -> Self { atomic::AtomicU8::new(val) }

    fn fetch_add(&self, new: u8, ordering: Ordering) -> u8 {
        self.fetch_add(new, ordering)
    }

    fn fetch_sub(&self, new: u8, ordering: Ordering) -> u8 {
        self.fetch_sub(new, ordering)
    }

    fn fetch_and(&self, new: u8, ordering: Ordering) -> u8 {
        self.fetch_and(new, ordering)
    }

    fn fetch_nand(&self, new: u8, ordering: Ordering) -> u8 {
        self.fetch_nand(new, ordering)
    }

    fn fetch_or(&self, new: u8, ordering: Ordering) -> u8 {
        self.fetch_or(new, ordering)
    }

    fn fetch_xor(&self, new: u8, ordering: Ordering) -> u8 {
        self.fetch_xor(new, ordering)
    }

    #[cfg(feature="nightly")]
    fn fetch_min(&self, val: u8, order: Ordering) -> u8 { self.fetch_min(val, order) }

    #[cfg(feature="nightly")]
    fn fetch_max(&self, val: u8, order: Ordering) -> u8 { self.fetch_max(val, order) }

    #[cfg(feature="nightly")]
    fn fetch_update<F>(
        &self,
        f: F,
        fetch_order: Ordering,
        set_order: Ordering
    ) -> Result<u8, u8>
    where F: FnMut(u8) -> Option<u8> {
        self.fetch_update(f, fetch_order, set_order)
    }

    fn get_mut(&mut self) -> &mut u8 { self.get_mut() }

    fn into_inner(self) -> u8 { self.into_inner() }

    fn load(&self, order: Ordering) -> u8 { self.load(order) }

    fn store(&self, val: u8, order: Ordering) { self.store(val, order) }

    fn swap(&self, val: u8, order: Ordering) -> u8 { self.swap(val, order) }

    fn compare_and_swap(
        &self,
        current: u8,
        new: u8,
        ordering: Ordering
    ) -> u8 {
        self.compare_and_swap(current, new, ordering)
    }

    fn compare_exchange(
        &self,
        current: u8,
        new: u8,
        success: Ordering,
        failure: Ordering
    ) -> Result<u8, u8> {
        self.compare_exchange(current, new, success, failure)
    }

    fn compare_exchange_weak(
        &self,
        current: u8,
        new: u8,
        success: Ordering,
        failure: Ordering
    ) -> Result<u8, u8> {
        self.compare_exchange_weak(current, new, success, failure)
    }

    #[cfg(feature="nightly")]
    fn as_mut_ptr(&self) -> *mut u8 { self.as_mut_ptr() }

}
impl AtomicInt for atomic::AtomicU16 {
    type Prim = u16;

    fn new(val: u16) -> Self { atomic::AtomicU16::new(val) }

    fn fetch_add(&self, new: u16, ordering: Ordering) -> u16 {
        self.fetch_add(new, ordering)
    }

    fn fetch_sub(&self, new: u16, ordering: Ordering) -> u16 {
        self.fetch_sub(new, ordering)
    }

    fn fetch_and(&self, new: u16, ordering: Ordering) -> u16 {
        self.fetch_and(new, ordering)
    }

    fn fetch_nand(&self, new: u16, ordering: Ordering) -> u16 {
        self.fetch_nand(new, ordering)
    }

    fn fetch_or(&self, new: u16, ordering: Ordering) -> u16 {
        self.fetch_or(new, ordering)
    }

    fn fetch_xor(&self, new: u16, ordering: Ordering) -> u16 {
        self.fetch_xor(new, ordering)
    }

    #[cfg(feature="nightly")]
    fn fetch_min(&self, val: u16, order: Ordering) -> u16 { self.fetch_min(val, order) }

    #[cfg(feature="nightly")]
    fn fetch_max(&self, val: u16, order: Ordering) -> u16 { self.fetch_max(val, order) }


    #[cfg(feature="nightly")]
    fn fetch_update<F>(
        &self,
        f: F,
        fetch_order: Ordering,
        set_order: Ordering
    ) -> Result<u16, u16>
    where F: FnMut(u16) -> Option<u16> {
        self.fetch_update(f, fetch_order, set_order)
    }

    fn get_mut(&mut self) -> &mut u16 { self.get_mut() }

    fn into_inner(self) -> u16 { self.into_inner() }

    fn load(&self, order: Ordering) -> u16 { self.load(order) }

    fn store(&self, val: u16, order: Ordering) { self.store(val, order) }

    fn swap(&self, val: u16, order: Ordering) -> u16 { self.swap(val, order) }

    fn compare_and_swap(
        &self,
        current: u16,
        new: u16,
        ordering: Ordering
    ) -> u16 {
        self.compare_and_swap(current, new, ordering)
    }

    fn compare_exchange(
        &self,
        current: u16,
        new: u16,
        success: Ordering,
        failure: Ordering
    ) -> Result<u16, u16> {
        self.compare_exchange(current, new, success, failure)
    }

    fn compare_exchange_weak(
        &self,
        current: u16,
        new: u16,
        success: Ordering,
        failure: Ordering
    ) -> Result<u16, u16> {
        self.compare_exchange_weak(current, new, success, failure)
    }

    #[cfg(feature="nightly")]
    fn as_mut_ptr(&self) -> *mut u16 { self.as_mut_ptr() }
}

impl AtomicInt for atomic::AtomicU32 {
    type Prim = u32;

    fn new(val: u32) -> Self { atomic::AtomicU32::new(val) }

    fn fetch_add(&self, new: u32, ordering: Ordering) -> u32 {
        self.fetch_add(new, ordering)
    }

    fn fetch_sub(&self, new: u32, ordering: Ordering) -> u32 {
        self.fetch_sub(new, ordering)
    }

    fn fetch_and(&self, new: u32, ordering: Ordering) -> u32 {
        self.fetch_and(new, ordering)
    }

    fn fetch_nand(&self, new: u32, ordering: Ordering) -> u32 {
        self.fetch_nand(new, ordering)
    }

    fn fetch_or(&self, new: u32, ordering: Ordering) -> u32 {
        self.fetch_or(new, ordering)
    }

    fn fetch_xor(&self, new: u32, ordering: Ordering) -> u32 {
        self.fetch_xor(new, ordering)
    }

    #[cfg(feature="nightly")]
    fn fetch_min(&self, val: u32, order: Ordering) -> u32 { self.fetch_min(val, order) }

    #[cfg(feature="nightly")]
    fn fetch_max(&self, val: u32, order: Ordering) -> u32 { self.fetch_max(val, order) }

    #[cfg(feature="nightly")]
    fn fetch_update<F>(
        &self,
        f: F,
        fetch_order: Ordering,
        set_order: Ordering
    ) -> Result<u32, u32>
    where F: FnMut(u32) -> Option<u32> {
        self.fetch_update(f, fetch_order, set_order)
    }

    fn get_mut(&mut self) -> &mut u32 { self.get_mut() }

    fn into_inner(self) -> u32 { self.into_inner() }

    fn load(&self, order: Ordering) -> u32 { self.load(order) }

    fn store(&self, val: u32, order: Ordering) { self.store(val, order) }

    fn swap(&self, val: u32, order: Ordering) -> u32 { self.swap(val, order) }

    fn compare_and_swap(
        &self,
        current: u32,
        new: u32,
        ordering: Ordering
    ) -> u32 {
        self.compare_and_swap(current, new, ordering)
    }

    fn compare_exchange(
        &self,
        current: u32,
        new: u32,
        success: Ordering,
        failure: Ordering
    ) -> Result<u32, u32> {
        self.compare_exchange(current, new, success, failure)
    }

    fn compare_exchange_weak(
        &self,
        current: u32,
        new: u32,
        success: Ordering,
        failure: Ordering
    ) -> Result<u32, u32> {
        self.compare_exchange_weak(current, new, success, failure)
    }

    #[cfg(feature="nightly")]
    fn as_mut_ptr(&self) -> *mut u32 { self.as_mut_ptr() }

}

impl AtomicInt for atomic::AtomicU64 {
    type Prim = u64;

    fn new(val: u64) -> Self { atomic::AtomicU64::new(val) }

    fn fetch_add(&self, new: u64, ordering: Ordering) -> u64 {
        self.fetch_add(new, ordering)
    }

    fn fetch_sub(&self, new: u64, ordering: Ordering) -> u64 {
        self.fetch_sub(new, ordering)
    }

    fn fetch_and(&self, new: u64, ordering: Ordering) -> u64 {
        self.fetch_and(new, ordering)
    }

    fn fetch_nand(&self, new: u64, ordering: Ordering) -> u64 {
        self.fetch_nand(new, ordering)
    }

    fn fetch_or(&self, new: u64, ordering: Ordering) -> u64 {
        self.fetch_or(new, ordering)
    }

    fn fetch_xor(&self, new: u64, ordering: Ordering) -> u64 {
        self.fetch_xor(new, ordering)
    }

    #[cfg(feature="nightly")]
    fn fetch_min(&self, val: u64, order: Ordering) -> u64 { self.fetch_min(val, order) }

    #[cfg(feature="nightly")]
    fn fetch_max(&self, val: u64, order: Ordering) -> u64 { self.fetch_max(val, order) }

    #[cfg(feature="nightly")]
    fn fetch_update<F>(
        &self,
        f: F,
        fetch_order: Ordering,
        set_order: Ordering
    ) -> Result<u64, u64>
    where F: FnMut(u64) -> Option<u64> {
        self.fetch_update(f, fetch_order, set_order)
    }

    fn get_mut(&mut self) -> &mut u64 { self.get_mut() }

    fn into_inner(self) -> u64 { self.into_inner() }

    fn load(&self, order: Ordering) -> u64 { self.load(order) }

    fn store(&self, val: u64, order: Ordering) { self.store(val, order) }

    fn swap(&self, val: u64, order: Ordering) -> u64 { self.swap(val, order) }

    fn compare_and_swap(
        &self,
        current: u64,
        new: u64,
        ordering: Ordering
    ) -> u64 {
        self.compare_and_swap(current, new, ordering)
    }

    fn compare_exchange(
        &self,
        current: u64,
        new: u64,
        success: Ordering,
        failure: Ordering
    ) -> Result<u64, u64> {
        self.compare_exchange(current, new, success, failure)
    }

    fn compare_exchange_weak(
        &self,
        current: u64,
        new: u64,
        success: Ordering,
        failure: Ordering
    ) -> Result<u64, u64> {
        self.compare_exchange_weak(current, new, success, failure)
    }

    #[cfg(feature="nightly")]
    fn as_mut_ptr(&self) -> *mut u64 { self.as_mut_ptr() }

}

impl AtomicInt for atomic::AtomicUsize {
    type Prim = usize;

    fn new(val: usize) -> Self { atomic::AtomicUsize::new(val) }

    fn fetch_add(&self, new: usize, ordering: Ordering) -> usize {
        self.fetch_add(new, ordering)
    }

    fn fetch_sub(&self, new: usize, ordering: Ordering) -> usize {
        self.fetch_sub(new, ordering)
    }

    fn fetch_and(&self, new: usize, ordering: Ordering) -> usize {
        self.fetch_and(new, ordering)
    }

    fn fetch_nand(&self, new: usize, ordering: Ordering) -> usize {
        self.fetch_nand(new, ordering)
    }

    fn fetch_or(&self, new: usize, ordering: Ordering) -> usize {
        self.fetch_or(new, ordering)
    }

    fn fetch_xor(&self, new: usize, ordering: Ordering) -> usize {
        self.fetch_xor(new, ordering)
    }

    #[cfg(feature="nightly")]
    fn fetch_min(&self, val: usize, order: Ordering) -> usize { self.fetch_min(val, order) }

    #[cfg(feature="nightly")]
    fn fetch_max(&self, val: usize, order: Ordering) -> usize { self.fetch_max(val, order) }

    #[cfg(feature="nightly")]
    fn fetch_update<F>(
        &self,
        f: F,
        fetch_order: Ordering,
        set_order: Ordering
    ) -> Result<usize, usize>
    where F: FnMut(usize) -> Option<usize> {
        self.fetch_update(f, fetch_order, set_order)
    }

    fn get_mut(&mut self) -> &mut usize { self.get_mut() }

    fn into_inner(self) -> usize { self.into_inner() }

    fn load(&self, order: Ordering) -> usize { self.load(order) }

    fn store(&self, val: usize, order: Ordering) { self.store(val, order) }

    fn swap(&self, val: usize, order: Ordering) -> usize { self.swap(val, order) }

    fn compare_and_swap(
        &self,
        current: usize,
        new: usize,
        ordering: Ordering
    ) -> usize {
        self.compare_and_swap(current, new, ordering)
    }

    fn compare_exchange(
        &self,
        current: usize,
        new: usize,
        success: Ordering,
        failure: Ordering
    ) -> Result<usize, usize> {
        self.compare_exchange(current, new, success, failure)
    }

    fn compare_exchange_weak(
        &self,
        current: usize,
        new: usize,
        success: Ordering,
        failure: Ordering
    ) -> Result<usize, usize> {
        self.compare_exchange_weak(current, new, success, failure)
    }

    #[cfg(feature="nightly")]
    fn as_mut_ptr(&self) -> *mut usize { self.as_mut_ptr() }

}

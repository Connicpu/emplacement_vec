#![feature(alloc, heap_api, placement_new_protocol)]

extern crate alloc;

use std::marker::PhantomData;
use std::ptr::read;
use std::ops::{Drop, Placer, Place, InPlace};
use std::mem::{transmute, align_of, size_of, forget};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::borrow::{Borrow, BorrowMut};
use alloc::heap::{allocate, reallocate, deallocate, EMPTY};

pub struct EmplacementVec<T> {
    data: *mut T,
    len: usize,
    cap: usize,
    phantom: PhantomData<T>,
}

impl<T> EmplacementVec<T> {
    pub fn new() -> Self {
        EmplacementVec {
            data: unsafe { transmute(EMPTY) },
            len: 0,
            cap: 0,
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn front(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { transmute(self.data) })
        }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { transmute(self.data) })
        }
    }

    pub fn back(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            let offset = (self.len - 1) as isize;
            Some(unsafe { transmute(self.data.offset(offset)) })
        }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            None
        } else {
            let offset = (self.len - 1) as isize;
            Some(unsafe { transmute(self.data.offset(offset)) })
        }
    }

    pub fn reserve(&mut self, capacity: usize) {
        let reserve_cap = self.len + capacity;
        self.grow_to(reserve_cap);
    }

    pub fn push(&mut self) -> PushPlace<T> {
        if self.len == self.cap {
            self.grow();
        }

        let pos = self.len as isize;
        self.len += 1;

        PushPlace {
            ptr: unsafe { self.data.offset(pos) },
            len: &mut self.len,
            phantom: PhantomData,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let pos = self.len as isize;
            unsafe {
                Some(read(self.data.offset(pos)))
            }
        }
    }

    fn elem_size(&self) -> usize {
        size_of::<T>()
    }

    fn elem_align(&self) -> usize {
        align_of::<T>()
    }

    fn grow(&mut self) {
        let new_cap = match self.cap {
            0 => 1,
            _ => self.cap,
        };

        self.grow_to(new_cap);
    }

    fn grow_to(&mut self, new_cap: usize) {
        if new_cap < self.cap {
            return
        }

        let old_cap = self.cap;
        self.cap = new_cap;

        if self.elem_size() != 0 {
            let new_size = self.cap * self.elem_size();
            let old_size = old_cap * self.elem_size();
            let align = self.elem_align();
            unsafe {
                if old_cap == 0 {
                    self.data = transmute(allocate(new_size, align));
                } else {
                    self.data = transmute(reallocate(
                        transmute(self.data), old_size, new_size, align
                    ));
                }
            }
        }
    }
}

impl<T> Borrow<[T]> for EmplacementVec<T> {
    fn borrow(&self) -> &[T] {
        unsafe { from_raw_parts(self.data, self.len) }
    }
}

impl<T> Drop for EmplacementVec<T> {
    fn drop(&mut self) {
        if self.elem_size() != 0 {
            let old_size = self.cap * self.elem_size();
            let align = self.elem_align();

            while self.len > 0 {
                self.pop();
            }

            unsafe {
                deallocate(transmute(self.data), old_size, align);
            }
        }
    }
}

pub struct PushPlace<'a, T> {
    ptr: *mut T,
    len: &'a mut usize,
    phantom: PhantomData<T>,
}

impl<'a, T> Placer<T> for PushPlace<'a, T> {
    type Place = Self;
    fn make_place(self) -> Self {
        self
    }
}

impl<'a, T> Place<T> for PushPlace<'a, T> {
    fn pointer(&mut self) -> *mut T {
        self.ptr
    }
}

impl<'a, T> InPlace<T> for PushPlace<'a, T> {
    type Owner = ();
    unsafe fn finalize(self) {
        forget(self)
    }
}

// If the Place doesn't get finalized, say if panic!() is called somehow,
// or if the user just called push() without `in` syntax, then we need to
// set len back.
impl<'a, T> Drop for PushPlace<'a, T> {
    fn drop(&mut self) {
        *self.len -= 1;
    }
}

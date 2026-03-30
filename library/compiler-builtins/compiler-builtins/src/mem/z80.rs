// Optimized memory functions for Z80/SM83.
//
// On Z80/SM83, usize is 16-bit and there is no benefit to word-at-a-time
// operations. The generic implementation's alignment checks and word-copy
// paths add ~280 bytes of overhead for zero performance gain.
// These simple byte loops produce ~15 bytes each.

use core::ffi::c_int;

#[inline(always)]
pub unsafe fn copy_forward(dest: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n {
        *dest.wrapping_add(i) = *src.wrapping_add(i);
        i += 1;
    }
}

#[inline(always)]
pub unsafe fn copy_backward(dest: *mut u8, src: *const u8, n: usize) {
    let mut i = n;
    while i > 0 {
        i -= 1;
        *dest.wrapping_add(i) = *src.wrapping_add(i);
    }
}

#[inline(always)]
pub unsafe fn set_bytes(s: *mut u8, c: u8, n: usize) {
    let mut i = 0;
    while i < n {
        *s.wrapping_add(i) = c;
        i += 1;
    }
}

#[inline(always)]
pub unsafe fn compare_bytes(s1: *const u8, s2: *const u8, n: usize) -> c_int {
    let mut i = 0;
    while i < n {
        let a = *s1.wrapping_add(i);
        let b = *s2.wrapping_add(i);
        if a != b {
            return (a as c_int) - (b as c_int);
        }
        i += 1;
    }
    0
}

#[inline(always)]
pub unsafe fn c_string_length(s: *const core::ffi::c_char) -> usize {
    let mut i = 0;
    while *s.wrapping_add(i) != 0 {
        i += 1;
    }
    i
}

// Z80/SM83 runtime builtins.
//
// The LLVM Z80 backend generates calls to these for 8-bit and 16-bit
// integer operations that SM83 doesn't have native instructions for.
// Naming follows the Z80 convention (__mulhi3 = 16-bit multiply, etc.)
// rather than the standard compiler-rt convention (__mulsi3 = 32-bit).

intrinsics! {
    // ── 16-bit multiply ─────────────────────────────────────────────────
    pub extern "C" fn __mulhi3(a: u16, b: u16) -> u16 {
        let mut result: u16 = 0;
        let mut multiplicand = a;
        let mut multiplier = b;
        while multiplier != 0 {
            if multiplier & 1 != 0 {
                result = result.wrapping_add(multiplicand);
            }
            multiplicand <<= 1;
            multiplier >>= 1;
        }
        result
    }

    // ── 32-bit multiply ──────────────────────────────────────────────────
    pub extern "C" fn __mulsi3(a: u32, b: u32) -> u32 {
        let mut result: u32 = 0;
        let mut multiplicand = a;
        let mut multiplier = b;
        while multiplier != 0 {
            if multiplier & 1 != 0 {
                result = result.wrapping_add(multiplicand);
            }
            multiplicand <<= 1;
            multiplier >>= 1;
        }
        result
    }

    // ── 16-bit upper multiply ───────────────────────────────────────────
    // Returns the upper 16 bits of a full 16×16→32 multiply.
    // Implemented with 8-bit components to avoid pulling in __mulsi3.
    //   a = (a_hi << 8) | a_lo,  b = (b_hi << 8) | b_lo
    //   a*b = a_hi*b_hi << 16
    //       + (a_hi*b_lo + a_lo*b_hi) << 8
    //       + a_lo*b_lo
    // We only need bits [31:16] of this sum.
    pub extern "C" fn __umulhi3(a: u16, b: u16) -> u16 {
        let a_lo = (a & 0xFF) as u16;
        let a_hi = (a >> 8) as u16;
        let b_lo = (b & 0xFF) as u16;
        let b_hi = (b >> 8) as u16;

        let lo_lo = a_lo.wrapping_mul(b_lo); // max 0xFE01, fits u16
        let hi_lo = a_hi.wrapping_mul(b_lo);
        let lo_hi = a_lo.wrapping_mul(b_hi);
        let hi_hi = a_hi.wrapping_mul(b_hi);

        // mid = hi_lo + lo_hi + carry from (lo_lo >> 8), tracked in 16-bit with carry
        let mid1 = hi_lo.wrapping_add(lo_hi);
        let mid_carry1: u16 = if mid1 < hi_lo { 1 } else { 0 };

        let lo_carry = lo_lo >> 8;
        let mid = mid1.wrapping_add(lo_carry);
        let mid_carry2: u16 = if mid < mid1 { 1 } else { 0 };

        hi_hi.wrapping_add(mid >> 8).wrapping_add((mid_carry1 + mid_carry2) << 8)
    }

    // ── 16-bit unsigned division ────────────────────────────────────────
    // Returns quotient. Remainder is discarded.
    pub extern "C" fn __udivhi3(dividend: u16, divisor: u16) -> u16 {
        if divisor == 0 { return 0; }
        let mut quotient: u16 = 0;
        let mut remainder: u16 = 0;
        let mut i: u8 = 16;
        let mut d = dividend;
        while i > 0 {
            remainder = (remainder << 1) | ((d >> 15) & 1);
            d <<= 1;
            if remainder >= divisor {
                remainder -= divisor;
                quotient = (quotient << 1) | 1;
            } else {
                quotient <<= 1;
            }
            i -= 1;
        }
        quotient
    }

    // ── 16-bit unsigned modulo ──────────────────────────────────────────
    pub extern "C" fn __umodhi3(dividend: u16, divisor: u16) -> u16 {
        if divisor == 0 { return 0; }
        let mut remainder: u16 = 0;
        let mut d = dividend;
        let mut i: u8 = 16;
        while i > 0 {
            remainder = (remainder << 1) | ((d >> 15) & 1);
            d <<= 1;
            if remainder >= divisor {
                remainder -= divisor;
            }
            i -= 1;
        }
        remainder
    }

    // ── 16-bit signed division ──────────────────────────────────────────
    pub extern "C" fn __divhi3(a: i16, b: i16) -> i16 {
        let neg = (a < 0) != (b < 0);
        let ua = if a < 0 { (-(a as i32)) as u16 } else { a as u16 };
        let ub = if b < 0 { (-(b as i32)) as u16 } else { b as u16 };
        let q = __udivhi3(ua, ub);
        if neg { -(q as i16) } else { q as i16 }
    }

    // ── 16-bit signed modulo ────────────────────────────────────────────
    pub extern "C" fn __modhi3(a: i16, b: i16) -> i16 {
        let neg = a < 0;
        let ua = if a < 0 { (-(a as i32)) as u16 } else { a as u16 };
        let ub = if b < 0 { (-(b as i32)) as u16 } else { b as u16 };
        let r = __umodhi3(ua, ub);
        if neg { -(r as i16) } else { r as i16 }
    }

    // ── 8-bit unsigned division ─────────────────────────────────────────
    pub extern "C" fn __udivqi3(dividend: u8, divisor: u8) -> u8 {
        if divisor == 0 { return 0; }
        let mut quotient: u8 = 0;
        let mut remainder: u8 = 0;
        let mut d = dividend;
        let mut i: u8 = 8;
        while i > 0 {
            remainder = (remainder << 1) | (d >> 7);
            d <<= 1;
            quotient <<= 1;
            if remainder >= divisor {
                remainder -= divisor;
                quotient |= 1;
            }
            i -= 1;
        }
        quotient
    }

    // ── 8-bit unsigned modulo ───────────────────────────────────────────
    pub extern "C" fn __umodqi3(dividend: u8, divisor: u8) -> u8 {
        if divisor == 0 { return 0; }
        let mut remainder: u8 = 0;
        let mut d = dividend;
        let mut i: u8 = 8;
        while i > 0 {
            remainder = (remainder << 1) | (d >> 7);
            d <<= 1;
            if remainder >= divisor {
                remainder -= divisor;
            }
            i -= 1;
        }
        remainder
    }
}

//! Module containing macros for implementing `core::fmt` traits.

use crate::int::I256;

macro_rules! impl_fmt {
    (for $int:ident) => {
        pub(crate) fn from_str_radix(src: &str, radix: u32) -> Result<$int, ParseIntError> {
            use core::num::IntErrorKind::*;
            use $crate::error::pie;

            assert!(
                (2..=36).contains(&radix),
                "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
                radix
            );

            if src.is_empty() {
                return Err(pie(Empty));
            }

            let is_signed_ty = $int::from_u32(0) > $int::min_value();

            // all valid digits are ascii, so we will just iterate over the utf8 bytes
            // and cast them to chars. .to_digit() will safely return None for anything
            // other than a valid ascii digit for the given radix, including the first-byte
            // of multi-byte sequences
            let src = src.as_bytes();

            let (is_positive, digits) = match src[0] {
                b'+' | b'-' if src[1..].is_empty() => {
                    return Err(pie(InvalidDigit));
                }
                b'+' => (true, &src[1..]),
                b'-' if is_signed_ty => (false, &src[1..]),
                _ => (true, src),
            };

            let mut result = $int::from_u32(0);
            if is_positive {
                // The number is positive
                for &c in digits {
                    let x = match (c as char).to_digit(radix) {
                        Some(x) => x,
                        None => return Err(pie(InvalidDigit)),
                    };
                    result = match result.checked_mul(radix) {
                        Some(result) => result,
                        None => return Err(pie(PosOverflow)),
                    };
                    result = match result.checked_add(x) {
                        Some(result) => result,
                        None => return Err(pie(PosOverflow)),
                    };
                }
            } else {
                // The number is negative
                for &c in digits {
                    let x = match (c as char).to_digit(radix) {
                        Some(x) => x,
                        None => return Err(pie(InvalidDigit)),
                    };
                    result = match result.checked_mul(radix) {
                        Some(result) => result,
                        None => return Err(pie(NegOverflow)),
                    };
                    result = match result.checked_sub(x) {
                        Some(result) => result,
                        None => return Err(pie(NegOverflow)),
                    };
                }
            }
            Ok(result)
        }

        impl ::core::str::FromStr for $int {
            type Err = ::core::num::ParseIntError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                from_str_radix(s, 10)
            }
        }

        impl ::core::fmt::Debug for U256 {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // NOTE: Work around `Formatter::debug_{lower,upper}_hex` being private
                // and not stabilized.
                #[allow(deprecated)]
                let flags = f.flags();
                const DEBUG_LOWER_HEX: u32 = 1 << 4;
                const DEBUG_UPPER_HEX: u32 = 1 << 5;

                if flags & DEBUG_LOWER_HEX != 0 {
                    fmt::LowerHex::fmt(self, f)
                } else if flags & DEBUG_UPPER_HEX != 0 {
                    fmt::UpperHex::fmt(self, f)
                } else {
                    fmt::Display::fmt(self, f)
                }
            }
        }

        fn $name(mut n: $u, is_nonnegative: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // 2^128 is about 3*10^38, so 39 gives an extra byte of space
            let mut buf = [MaybeUninit::<u8>::uninit(); 39];
            let mut curr = buf.len() as isize;
            let buf_ptr = MaybeUninit::slice_as_mut_ptr(&mut buf);
            let lut_ptr = DEC_DIGITS_LUT.as_ptr();

            // SAFETY: Since `d1` and `d2` are always less than or equal to `198`, we
            // can copy from `lut_ptr[d1..d1 + 1]` and `lut_ptr[d2..d2 + 1]`. To show
            // that it's OK to copy into `buf_ptr`, notice that at the beginning
            // `curr == buf.len() == 39 > log(n)` since `n < 2^128 < 10^39`, and at
            // each step this is kept the same as `n` is divided. Since `n` is always
            // non-negative, this means that `curr > 0` so `buf_ptr[curr..curr + 1]`
            // is safe to access.
            unsafe {
                // need at least 16 bits for the 4-characters-at-a-time to work.
                assert!(crate::mem::size_of::<$u>() >= 2);

                // eagerly decode 4 characters at a time
                while n >= 10000 {
                    let rem = (n % 10000) as isize;
                    n /= 10000;

                    let d1 = (rem / 100) << 1;
                    let d2 = (rem % 100) << 1;
                    curr -= 4;

                    // We are allowed to copy to `buf_ptr[curr..curr + 3]` here since
                    // otherwise `curr < 0`. But then `n` was originally at least `10000^10`
                    // which is `10^40 > 2^128 > n`.
                    ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                    ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
                }

                // if we reach here numbers are <= 9999, so at most 4 chars long
                let mut n = n as isize; // possibly reduce 64bit math

                // decode 2 more chars, if > 2 chars
                if n >= 100 {
                    let d1 = (n % 100) << 1;
                    n /= 100;
                    curr -= 2;
                    ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                }

                // decode last 1 or 2 chars
                if n < 10 {
                    curr -= 1;
                    *buf_ptr.offset(curr) = (n as u8) + b'0';
                } else {
                    let d1 = n << 1;
                    curr -= 2;
                    ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                }
            }

            // SAFETY: `curr` > 0 (since we made `buf` large enough), and all the chars are valid
            // UTF-8 since `DEC_DIGITS_LUT` is
            let buf_slice = unsafe {
                str::from_utf8_unchecked(
                    slice::from_raw_parts(buf_ptr.offset(curr), buf.len() - curr as usize))
            };
            f.pad_integral(is_nonnegative, "", buf_slice)
        }

        impl ::core::fmt::Display for $int {
            #[allow(unused_comparisons)]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                use $crate::uint::AsU256;

                let is_nonnegative = *self >= 0;
                let n = if is_nonnegative {
                    self.as_u256()
                } else {
                    // convert the negative num to positive by summing 1 to it's 2 complement
                    (!self.as_u256()).wrapping_add(1)
                };
                (n, is_nonnegative, f)
            }
        }
    };
}

impl_fmt! {
    for I256
}

const DEC_DIGITS_LUT: &[u8; 200] = b"\
    0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut n = *self;

        // 2^256 is about 1*10^78, so 79 gives an extra byte of space
        let mut buf = [MaybeUninit::<u8>::uninit(); 79];
        let mut curr = buf.len() as isize;
        let buf_ptr = &mut buf[0] as *mut _ as *mut u8;
        let lut_ptr = DEC_DIGITS_LUT.as_ptr();

        // SAFETY: Since `d1` and `d2` are always less than or equal to `198`, we
        // can copy from `lut_ptr[d1..d1 + 1]` and `lut_ptr[d2..d2 + 1]`. To show
        // that it's OK to copy into `buf_ptr`, notice that at the beginning
        // `curr == buf.len() == 39 > log(n)` since `n < 2^128 < 10^39`, and at
        // each step this is kept the same as `n` is divided. Since `n` is always
        // non-negative, this means that `curr > 0` so `buf_ptr[curr..curr + 1]`
        // is safe to access.
        unsafe {
            // eagerly decode 4 characters at a time
            while n >= 10000 {
                let rem = *(n % 10000).low() as isize;
                n /= 10000;

                let d1 = (rem / 100) << 1;
                let d2 = (rem % 100) << 1;
                curr -= 4;

                // We are allowed to copy to `buf_ptr[curr..curr + 3]` here since
                // otherwise `curr < 0`. But then `n` was originally at least `10000^10`
                // which is `10^40 > 2^128 > n`.
                ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
            }

            // if we reach here numbers are <= 9999, so at most 4 chars long
            let mut n = *n.low() as isize; // possibly reduce 64bit math

            // decode 2 more chars, if > 2 chars
            if n >= 100 {
                let d1 = (n % 100) << 1;
                n /= 100;
                curr -= 2;
                ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
            }

            // decode last 1 or 2 chars
            if n < 10 {
                curr -= 1;
                *buf_ptr.offset(curr) = (n as u8) + b'0';
            } else {
                let d1 = n << 1;
                curr -= 2;
                ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
            }
        }

        // SAFETY: `curr` > 0 (since we made `buf` large enough), and all the chars are valid
        // UTF-8 since `DEC_DIGITS_LUT` is
        let buf_slice = unsafe {
            str::from_utf8_unchecked(slice::from_raw_parts(
                buf_ptr.offset(curr),
                buf.len() - curr as usize,
            ))
        };
        f.pad_integral(true, "", buf_slice)
    }
}

pub(crate) fn fmt_radix(
    mut x: U256,
    base: usize,
    prefix: &str,
    digits: &[u8],
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let mut buf = [MaybeUninit::<u8>::uninit(); 256];
    let mut curr = buf.len();

    // Accumulate each digit of the number from the least significant
    // to the most significant figure.
    for byte in buf.iter_mut().rev() {
        let n = (*x.low() as usize) % base;
        x /= base.as_u256(); // Deaccumulate the number.
        #[cfg(debug_assertions)]
        let digit = digits[n];
        #[cfg(not(debug_assertions))]
        let digit = unsafe { *digits.get_unchecked(n) };
        byte.write(digit); // Store the digit in the buffer.
        curr -= 1;
        if x == 0 {
            // No more digits left to accumulate.
            break;
        };
    }
    let buf = &buf[curr..];

    // SAFETY: The only chars in `buf` are created by `Self::digit` which are assumed to be
    // valid UTF-8
    let buf = unsafe { str::from_utf8_unchecked(&*(buf as *const _ as *const [u8])) };
    f.pad_integral(true, prefix, buf)
}

impl fmt::Binary for U256 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_radix(*self, 2, "0b", b"01", f)
    }
}

impl fmt::Octal for U256 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_radix(*self, 8, "0o", b"01234567", f)
    }
}

impl fmt::LowerHex for U256 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_radix(*self, 16, "0x", b"0123456789abcdef", f)
    }
}

impl fmt::UpperHex for U256 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_radix(*self, 16, "0x", b"0123456789ABCDEF", f)
    }
}

impl fmt::LowerExp for U256 {
    #[allow(unused_comparisons)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(nlordell): Ideally this should be implemented with a similar
        // to the primitive integer types as seen here:
        // https://doc.rust-lang.org/src/core/fmt/num.rs.html#274
        // Unfortunately, just porting this implementation is not possible as it
        // requires private standard library items. For now, just convert to
        // a `f64` as an approximation.
        fmt::LowerExp::fmt(&self.as_f64(), f)
    }
}

impl fmt::UpperExp for U256 {
    #[allow(unused_comparisons)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperExp::fmt(&self.as_f64(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", U256::MAX),
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
        );
        assert_eq!(
            format!("{:x?}", U256::MAX),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        );
        assert_eq!(
            format!("{:#X?}", U256::MAX),
            "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", U256::MAX),
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
        );
    }

    #[test]
    fn radix() {
        assert_eq!(format!("{:b}", U256::new(42)), "101010");
        assert_eq!(format!("{:o}", U256::new(42)), "52");
        assert_eq!(format!("{:x}", U256::new(42)), "2a");
    }

    #[test]
    fn exp() {
        assert_eq!(format!("{:e}", U256::new(42)), "4.2e1");
        assert_eq!(format!("{:e}", U256::new(10).pow(77)), "1e77");
        assert_eq!(format!("{:E}", U256::new(10).pow(39) * 1337), "1.337E42");
    }

    #[test]
    fn errors() {
        assert_eq!(
            U256::from_str_radix("", 2).unwrap_err().kind(),
            &IntErrorKind::Empty,
        );
        assert_eq!(
            U256::from_str_radix("?", 2).unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
        assert_eq!(
            U256::from_str_radix("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz", 36)
                .unwrap_err()
                .kind(),
            &IntErrorKind::PosOverflow,
        );
    }
}
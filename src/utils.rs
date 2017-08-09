use std::mem::replace;
use num_traits::ops::wrapping::WrappingSub;
use num_traits::{
    NumRef,
    PrimInt,
    Unsigned,
};

pub fn gcd<T>(mut a: T, mut b: T) -> T
    where T: PrimInt + NumRef + WrappingSub + Copy
{
    loop {
        a = a % &b; // Put reminder to `a`

        if a.is_zero() {
            if b < T::zero() { // Negate and return
                break T::zero().wrapping_sub(&b);
            }
            break b;
        }

        a = replace(&mut b, a);
    }
}

pub fn mod_pow<T>(mut value: T, mut power: T, modulus: T) -> T
    where T: PrimInt + Unsigned
{
    let mut result: T = T::one();

    while !power.is_zero() {
        if power & T::one() == T::one() {
            result = (result * value) % modulus;
        }
        if power != T::one() {
            value = (value * value) % modulus;
        }
        power = power >> 1;
    }
    result % modulus
}

#[allow(dead_code)]
pub fn mod_pow_checked<T>(mut value: T, mut power: T, modulus: T) -> Option<T>
    where T: PrimInt
{
    if value < T::zero()
        || power < T::zero()
            || modulus < T::zero() {
        return None;
    }

    let mut result: T = T::one();

    while !power.is_zero() {

        if power & T::one() == T::one() {
            match result.checked_mul(&value) {
                Some(product) => result = product % modulus,
                None => return None,
            }
        }

        if power != T::one() {
            match value.checked_mul(&value) {
                Some(product) => value = product % modulus,
                None => return None,
            }
        }

        power = power >> 1;
    }
    Some(result % modulus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_pow_unchecked_asserts() {
        assert_eq!(mod_pow(5, 2, 7), 4_u8);
        assert_eq!(mod_pow(500, 2000, 50323), 12847_u32);
        assert_eq!(mod_pow(37527821, 18764, 234234), 991_u64);
        assert_eq!(mod_pow(3752127821, 18764, 234234), 28561_u64);

    }

    #[test]
    #[should_panic]
    fn mod_pow_unchecked_overflow_asserts() {
        assert_eq!(mod_pow(37527821, 18764, 234234), 991_u32);
        assert_eq!(mod_pow(3752127821, 18764, 234234), 28561_u32);
    }

    #[test]
    fn mod_pow_checked_asserts() {
        assert_eq!(mod_pow_checked(-1,  1,  1), None);
        assert_eq!(mod_pow_checked( 1, -1,  1), None);
        assert_eq!(mod_pow_checked( 1,  1, -1), None);

        assert_eq!(mod_pow_checked(5, 2, 7_i8), Some(4));
        assert_eq!(mod_pow_checked(500, 2000, 50323_i32), None);
        assert_eq!(mod_pow_checked(500, 2000, 50323_i64), Some(12847));
        assert_eq!(mod_pow_checked(37527821, 18764, 234234), None);
        assert_eq!(mod_pow_checked(37527821, 18764, 234234_i64), Some(991));
        assert_eq!(mod_pow_checked(3752127821, 18764, 234234_i64), None);
        assert_eq!(mod_pow_checked(3752127821, 18764, 234234_u64), Some(28561));
    }

    #[test]
    fn gcd_asserts() {
        assert_eq!(gcd(1, 27), 1);
        assert_eq!(gcd(121, 45), 1);
        assert_eq!(gcd(90, 36), 18);
        assert_eq!(gcd(21, 217), 7);
        assert_eq!(gcd(-12, 6), 6);
        assert_eq!(gcd(12, -6), 6);
        assert_eq!(gcd(-12, -6), 6);
        assert_eq!(gcd( -<i64>::max_value(), 7), 7);
    }
}

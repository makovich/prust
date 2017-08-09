use utils::mod_pow;
use jacobi_symbol::jacobi_symbol;
use rand::{
    Rng,
    thread_rng
};

/// SOLOVAY-STRASSEN(n, t)
/// INPUT: an odd integer `n >= 3` and security parameter `t >= 1`.
/// OUTPUT: an answer to the question:  "Is `n` prime?"
///  1. For `i` from 1 to `t` do the following:
///   1.1 Choose a random integer `a`, `a in [2..n-2]`.
///   1.2 Compute `r = a^((n-1)/2) mod n`.
///   1.3 If `r != 1` and `r != n-1` then return ("composite").
///   1.4 Compute the Jacobi symbol `s = (a/n)`.
///   1.5 If `r !== s (mod n)` then return ("composite").
///  2. return("prime").
pub fn is_prime(candidate: u64) -> bool {
    solovay_strassen(candidate)
}

fn solovay_strassen(candidate: u64) -> bool {
    match candidate {
        0 | 1 => false,
            2 => true,
            3 => true,
            _ => solovay_strassen_core(candidate, 3),
    }
}

fn solovay_strassen_core(candidate: u64, attempts: u8) -> bool {
    for _ in 0..attempts {
        let a = thread_rng().gen_range(2, candidate - 1);
        let r = mod_pow(a, ((candidate - 1) / 2), candidate);

        if r != 1 && r != candidate - 1 {
            return false;
        }

        let s = jacobi_symbol(a, candidate);

        // Make Jacobi symbol positive for the following Rem (%) operation
        let s = (s + candidate as i64) as u64;

        if r != s % candidate {
            return false;
        }
    }
    // Probably prime number
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solovay_strassen_for_border_cases() {
        assert_eq!(solovay_strassen(0), false);
        assert_eq!(solovay_strassen(1), false);
        assert_eq!(solovay_strassen(2), true);
    }

    #[test]
    fn solovay_strassen_for_primes() {
        assert_eq!(solovay_strassen(3), true);
        assert_eq!(solovay_strassen(5), true);
        assert_eq!(solovay_strassen(11), true);
        assert_eq!(solovay_strassen(307), true);
        assert_eq!(solovay_strassen(839), true);
        assert_eq!(solovay_strassen(1151), true);
        assert_eq!(solovay_strassen(10499), true);
        assert_eq!(solovay_strassen(423425347), true);
    }

    #[test]
    fn solovay_strassen_for_composites() {
        assert_eq!(solovay_strassen(4), false);
        assert_eq!(solovay_strassen(15), false);
        assert_eq!(solovay_strassen(155), false);
        assert_eq!(solovay_strassen(400), false);
        assert_eq!(solovay_strassen(4009), false);
    }
}

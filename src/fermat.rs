use utils::mod_pow;
use rand::{
    Rng,
    thread_rng
};

/// FERMAT(n, t)
/// INPUT: an odd integer `n >= 3` and security parameter `t >= 1`.
/// OUTPUT: an answer to the question:  "Is `n` prime?"
///  1. For `i` from 1 to `t` do the following:
///   1.1 Choose a random integer `a`, `a in [2..n-2]`.
///   1.2 Compute `r = a^n-1 mod n`.
///   1.3 If `r != 1` then return ("composite").
///  2. return("prime").
pub fn is_prime(candidate: u64) -> bool {
    fermat_test(candidate)
}

fn fermat_test(candidate: u64) -> bool {
    match candidate {
        0 | 1 => false,
            2 => true,
            3 => true,
            _ => fermat_test_core(candidate, 3),
    }
}

fn fermat_test_core(candidate: u64, attempts: u8) -> bool {
    for _ in 0..attempts {
        let a = thread_rng().gen_range(2, candidate - 1);
        let r = mod_pow(a, candidate - 1, candidate);

        if r != 1 {
            // Definitely composite number
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
    fn fermat_test_for_border_cases() {
        assert_eq!(fermat_test(0), false);
        assert_eq!(fermat_test(1), false);
        assert_eq!(fermat_test(2), true);
    }

    #[test]
    fn fermat_test_for_primes() {
        assert_eq!(fermat_test(3), true);
        assert_eq!(fermat_test(5), true);
        assert_eq!(fermat_test(11), true);
        assert_eq!(fermat_test(307), true);
        assert_eq!(fermat_test(839), true);
        assert_eq!(fermat_test(1151), true);
        assert_eq!(fermat_test(10499), true);
        assert_eq!(fermat_test(423425347), true);
    }

    #[test]
    fn fermat_test_for_composites() {
        assert_eq!(fermat_test(4), false);
        assert_eq!(fermat_test(15), false);
        assert_eq!(fermat_test(155), false);
        assert_eq!(fermat_test(400), false);
        assert_eq!(fermat_test(4009), false);
    }
}

use utils::mod_pow;
use rand::{
    Rng,
    thread_rng
};

// MILLER-RABIN(n, t)
// INPUT: an odd integer `n >= 3` and security parameter `t >= 1`.
// OUTPUT: an answer to the question:  "Is `n` prime?"
pub fn is_prime(candidate: u64) -> bool {
    miller_rabin(candidate)
}

fn miller_rabin(candidate: u64) -> bool {
    match candidate {
        0 | 1 => false,
            2 => true,
            3 => true,
            n if n & 1 == 0 => false,
            _ =>  miller_rabin_core(candidate, 3),
    }
}

fn miller_rabin_core(candidate: u64, attempts: u8) -> bool {
    let mut r = 0;
    let mut d = candidate - 1;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    for _ in 0..attempts {
        let a = thread_rng().gen_range(2, candidate - 1);
        let mut x = mod_pow(a, d, candidate);

        if x == 1 // First equality (a^d mod n === 1)
            || x == candidate - 1 { // Second equality (a^2^r mod n === -1, where r = 0)
                continue;
        }

        for _ in 0..r { // repeat `r-1` times
            x = mod_pow(x, 2, candidate);

            if x == candidate - 1 {
                break;
            }
        }

        // No one `r` satisfied second equality
        if x != candidate - 1 {
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
    fn miller_rabin_for_border_cases() {
        assert_eq!(miller_rabin(0), false);
        assert_eq!(miller_rabin(1), false);
        assert_eq!(miller_rabin(2), true);
    }

    #[test]
    fn miller_rabin_for_primes() {
        assert_eq!(miller_rabin(3), true);
        assert_eq!(miller_rabin(5), true);
        assert_eq!(miller_rabin(11), true);
        assert_eq!(miller_rabin(307), true);
        assert_eq!(miller_rabin(839), true);
        assert_eq!(miller_rabin(1151), true);
        assert_eq!(miller_rabin(10499), true);
        assert_eq!(miller_rabin(423425347), true);
        assert_eq!(miller_rabin(4194304903), true);
    }

    #[test]
    fn miller_rabin_for_composites() {
        assert_eq!(miller_rabin(4), false);
        assert_eq!(miller_rabin(15), false);
        assert_eq!(miller_rabin(155), false);
        assert_eq!(miller_rabin(400), false);
        assert_eq!(miller_rabin(4009), false);
    }

    #[test]
    #[ignore]
    fn miller_rabin_for_strong_pseudoprimes() {
        assert!(miller_rabin(121) == false);
        assert!(miller_rabin(703) == false);
        assert!(miller_rabin(1891) == false);
        assert!(miller_rabin(3281) == false);
        assert!(miller_rabin(8321) == false);
        assert!(miller_rabin(15841) == false);
        assert!(miller_rabin(29341) == false);
        assert!(miller_rabin(42799) == false);
        assert!(miller_rabin(49141) == false);
        assert!(miller_rabin(52633) == false);
        assert!(miller_rabin(65281) == false);
        assert!(miller_rabin(74665) == false);
    }
}

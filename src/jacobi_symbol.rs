use utils::gcd;

pub fn jacobi_symbol(a: u64, n: u64) -> i64 {
    jacobi_symbol_rec(a as i64, n as i64) as i64
}

/// http://zoo.cs.yale.edu/classes/cs467/2010s/handouts/ho07.pdf
pub fn jacobi_symbol_rec(a: i64, n: i64) -> i8 {
    match (a, n) {
        // Theorem 6.1
        (0, n) if n == 1 => 1,
        (0, n) if n > 1 => 0,

        // Theorem 6.2
        (2, n) if ( n % 8 == 1 || n % 8 == 7 ) => 1,
        (2, n) if ( n % 8 == 3 || n % 8 == 5 ) => -1,

        // Theorem 6.3
        (a, n) if a >= n => jacobi_symbol_rec(a % n, n),

        // Theorem 6.4
        (a, n) if a % 2 == 0 => jacobi_symbol_rec(2, n) * jacobi_symbol_rec(a / 2, n),

        // Theorem 6.5
        (a, n) if ( a % 4 == 3 && n % 4 == 3 ) => -1 * jacobi_symbol_rec(n, a),
        (a, n) => jacobi_symbol_rec(n, a),
    }
}

/// https://ru.wikipedia.org/wiki/%D0%A1%D0%B8%D0%BC%D0%B2%D0%BE%D0%BB_%D0%AF%D0%BA%D0%BE%D0%B1%D0%B8
#[allow(dead_code)]
pub fn jacobi_symbol_loop(mut a: i64, mut n: i64) -> i8 {
    if gcd(a, n) != 1 {
        return 0;
    }

    let mut s = 1_i8;

    if a < 0 {
        a = -a;
        if n % 4 == 3 {
            s = -s;
        }
    }

    loop {
        let mut t = 0;

        while a % 2 == 0 {
            t += 1;
            a /= 2;
        }

        if t % 2 != 0 {
            if n % 8 == 3 || n % 8 == 5 {
                s = -s;
            }
        }

        if a % 4 == 3 && n % 4 == 3 {
            s = -s;
        }

        let tmp = a;
        a = n % tmp;
        n = tmp;

        if a == 0 {
            break s;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct JacobiTestCase(i8, i64, i64);

    fn get_jacobi_test_cases() -> Vec<JacobiTestCase> {
        vec![
            JacobiTestCase(-1, 1001,   9907),
            JacobiTestCase( 1,  219,    383),
            JacobiTestCase( 1, 1236,  20003),
            JacobiTestCase(-1,   23,   1343),
            JacobiTestCase( 1, 2423, 123123),
        ]
    }

    #[test]
    fn jacobi_symbol_recursive_function() {
        for c in get_jacobi_test_cases() {
            assert!(c.0 == jacobi_symbol_rec(c.1, c.2),
                format!("a={}, n={}, s={}", c.1, c.2, c.0));
        }
    }

    #[test]
    fn jacobi_symbol_loop_based_function() {
        for c in get_jacobi_test_cases() {
            assert!(c.0 == jacobi_symbol_loop(c.1, c.2),
                format!("a={}, n={}, s={}", c.1, c.2, c.0));
        }
    }
}

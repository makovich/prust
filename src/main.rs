extern crate prust;
extern crate ansi_term;

use ansi_term::Colour::{
    Red,
    Green,
    White
};

use prust::{
    fermat,
    miller_rabin,
    solovay_strassen
};

fn main() {
    for tc in get_prime_test_cases() {
        let f = fermat::is_prime(tc.1);
        let ss = solovay_strassen::is_prime(tc.1);
        let mr = miller_rabin::is_prime(tc.1);

        let fmt = |expected, result| if expected == result { Green.paint("+") } else { Red.paint("-") };

        println!("{} is a {}:",
            White.paint(tc.1.to_string()),
            White.paint(if tc.0 { "prime" } else { "composite" }));
        println!("  {} Fermat", fmt(tc.0, f));
        println!("  {} Solovay-Strassen", fmt(tc.0, ss));
        println!("  {} Miller-Rabin", fmt(tc.0, mr));
        println!();
    }
}

fn get_prime_test_cases() -> Vec<(bool, u64)> {
    vec![
        // Border cases
        (false, 0),
        (false, 1),
        (true,  2),

        // Primes
        (true, 3),
        (true, 5),
        (true, 11),
        (true, 307),
        (true, 839),
        (true, 1151),
        (true, 10499),
        (true, 423425347),

        // Non-primes
        (false, 4),
        (false, 15),
        (false, 155),
        (false, 400),
        (false, 4009),

        // Pseudoprimes
        (false, 121),
        (false, 703 ),
        (false, 1891),
        (false, 3281),
        (false, 8321),
        (false, 15841),
        (false, 29341),
        (false, 42799),
        (false, 49141),
        (false, 52633),
        (false, 65281),
        (false, 74665),
    ]
}

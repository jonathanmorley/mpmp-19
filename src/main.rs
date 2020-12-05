use primal::Primes;
use num_bigint::BigUint;
use num_traits::identities::Zero;
use num_format::{Locale, ToFormattedString};

fn main() {
    let iter = Primes::all()
        .scan(BigUint::zero(), |state, x| {
            *state += x * x;
            Some(state.clone())
        })
        .zip(1u128..)
        .inspect(|(_, idx)| {
            print!("\rChecking prime #{}", idx.to_formatted_string(&Locale::en));
        })
        .filter(|(x, idx)| x % idx == BigUint::zero());

    for (x, idx) in iter {
        print!("\r                                                                                                                 \r{} ({})\n", idx.to_formatted_string(&Locale::en), x);
    }
}

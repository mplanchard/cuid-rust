use rand::{thread_rng, CryptoRng, Rng};

use crate::error::CuidError;
use crate::text::{pad, to_base_string};
use crate::{BLOCK_SIZE, DISCRETE_VALUES};

fn random_float_from_rng<R: Rng + CryptoRng>(mut rng: R) -> f64 {
    rng.gen::<f64>()
}

fn random_float() -> f64 {
    random_float_from_rng(thread_rng())
}

fn random_64_bit_int<N: Into<f64>>(max: N) -> u64 {
    (random_float() * max.into()) as u64
}

pub fn random_block() -> Result<String, CuidError> {
    to_base_string(random_64_bit_int(DISCRETE_VALUES as u32)).map(|s| pad(BLOCK_SIZE, s))
}

#[cfg(test)]
mod test_randoms {
    use super::*;

    #[test]
    fn random_block_len() {
        assert_eq!(random_block().unwrap().len(), BLOCK_SIZE);
    }

    // TODO: This is theoretically a bit brittle?
    #[test]
    fn multiple_blocks_not_equal() {
        assert!(random_block().unwrap() != random_block().unwrap())
    }
}

#[cfg(nightly)]
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::u32;
    use test::Bencher;

    #[bench]
    fn bench_random_float(b: &mut Bencher) {
        b.iter(|| random_float())
    }

    #[bench]
    fn bench_random_64_bit_int(b: &mut Bencher) {
        // this shouldn't take noticeably more time than generating a
        // random float
        b.iter(|| random_64_bit_int(u32::MAX))
    }

    #[bench]
    fn bench_random_block(b: &mut Bencher) {
        b.iter(|| random_block())
    }
}

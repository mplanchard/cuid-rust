use rand::{thread_rng, CryptoRng, Rng};

use super::{BLOCK_SIZE, DISCRETE_VALUES};
use error::CuidError;
use text::{pad, to_base_str};

fn random_float_from_rng<R: Rng + CryptoRng>(mut rng: R) -> f64 {
    rng.gen::<f64>()
}

fn random_float() -> f64 {
    random_float_from_rng(thread_rng())
}

fn random_64_bit_int<N: Into<f64>>(max: N) -> u64 {
    (random_float() * max.into()) as u64
}

pub fn random_block() -> Result<Box<str>, CuidError> {
    to_base_str(random_64_bit_int(DISCRETE_VALUES as u32)).map(|s| pad(BLOCK_SIZE as u32, &s))
}

#[cfg(test)]
mod test_randoms {
    use super::*;

    #[test]
    fn random_block_len() {
        assert!(random_block().unwrap().len() == BLOCK_SIZE as usize)
    }

    // TODO: This is theoretically a bit brittle?
    #[test]
    fn multiple_blocks_not_equal() {
        assert!(random_block().unwrap() != random_block().unwrap())
    }

}

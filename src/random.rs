use rand::{ CryptoRng, Rng, thread_rng };

use super::{ BLOCK_SIZE, DISCRETE_VALUES };
use text::{ pad, to_base_str };

// The default implemention is cryptographically secure, as it
// is a shortcut for thread_rng().gen(), and ThreadRng implements
// the CryptoRng trait.


fn random_float_from_rng<R: Rng + CryptoRng>(mut rng: R) -> f64 {
    rng.gen::<f64>()
}


fn random_float() -> f64 {
    random_float_from_rng(thread_rng())
}


fn random_32bit_int<N: Into<f64>>(max: N) -> u32 {
    (random_float() * max.into()) as u32
}


pub fn random_block() -> String {
    pad(
        BLOCK_SIZE as u32,
        to_base_str(random_32bit_int(DISCRETE_VALUES as u32))
    )
}


#[cfg(test)]
mod test_randoms {
    use super::*;

    #[test]
    fn random_block_len() {
        assert!(random_block().len() == BLOCK_SIZE as usize)
    }

    // TODO: This is theoretically a bit brittle?
    #[test]
    fn multiple_blocks_not_equal() {
        assert!(random_block() != random_block())
    }

}

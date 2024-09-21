use rand::{distributions::Standard, prelude::Distribution, CryptoRng, Error, Rng, RngCore};

use crate::ffi;

pub fn random<T>() -> T
where
    Standard: Distribution<T>,
{
    System.gen()
}


// Derived Copy, Debug, and Default have no effect since the RNG's internal state is stored outside
// of the crate.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct System;

impl RngCore for System {
    fn next_u32(&mut self) -> u32 {
        unsafe { ffi::rng::random_32() }
    }

    fn next_u64(&mut self) -> u64 {
        unsafe { ffi::rng::random_64() }
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        unsafe { ffi::rng::random_bytes(dest.as_mut_ptr(), dest.len()) }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for System {}

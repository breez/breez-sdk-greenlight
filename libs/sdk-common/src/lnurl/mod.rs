pub mod error;
pub mod model;
pub mod specs;

#[cfg(test)]
mod tests {
    use bitcoin::secp256k1::rand;
    use bitcoin::secp256k1::rand::distributions::{Alphanumeric, DistString};

    pub fn rand_string(len: usize) -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), len)
    }
}

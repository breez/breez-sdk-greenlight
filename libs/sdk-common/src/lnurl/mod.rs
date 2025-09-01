pub mod error;
pub mod model;
pub mod specs;

#[cfg(test)]
mod tests {
    use rand::Rng;
    pub fn rand_string(len: usize) -> String {
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}

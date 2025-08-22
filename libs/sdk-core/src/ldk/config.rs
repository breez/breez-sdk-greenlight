pub(crate) struct Config {
    pub esplora_url: &'static str,
    pub rgs_url: &'static str,
    pub lsps2_id: &'static str,
    pub lsps2_address: &'static str,
}

impl Config {
    pub fn regtest() -> Self {
        Self {
            esplora_url: "http://localhost:30000",
            rgs_url: "http://localhost:8011/v2",
            lsps2_id: "02b49b94e068e05c04c2ac98e096a06202d04920daec25d82f7898e21901f15d81",
            lsps2_address: "localhost:9735",
        }
    }
}

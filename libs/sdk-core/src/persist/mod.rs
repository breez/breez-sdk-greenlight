pub(crate) mod cache;
pub(crate) mod channels;
pub(crate) mod db;
pub(crate) mod migrations;
pub(crate) mod settings;
pub(crate) mod swap;
pub(crate) mod transactions;

#[cfg(test)]
mod test_utils {
    use rand::Rng;

    pub fn create_test_sql_file(suffix: String) -> String {
        let mut tmp_file = std::env::temp_dir();
        let mut rng = rand::thread_rng();
        tmp_file.push(format!("test_{}{}.sql", suffix, rng.gen::<u32>()));
        let path = tmp_file.as_path();
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        let s = path.to_str().unwrap();
        String::from(s)
    }
}

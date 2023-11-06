use anyhow::{anyhow, Result};

pub fn encrypt(key: Vec<u8>, msg: Vec<u8>) -> Result<Vec<u8>> {
    match ecies::encrypt(key.as_slice(), msg.as_slice()) {
        Ok(res) => Ok(res),
        Err(err) => Err(anyhow!(err.to_string())),
    }
}

#[allow(dead_code)]
pub fn decrypt(key: Vec<u8>, msg: Vec<u8>) -> Result<Vec<u8>> {
    match ecies::decrypt(key.as_slice(), msg.as_slice()) {
        Ok(res) => Ok(res),
        Err(err) => Err(anyhow!(err.to_string())),
    }
}

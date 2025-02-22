use js_sys::*;
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::*;

#[wasm_bindgen(module = "/mock_service_worker.js")]
extern "C" {}

#[derive(Debug, Serialize)]
struct MockConfig {
    pub kind: String,
    pub method: String,
    pub path: String,
    pub status_code: u16,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

pub(crate) struct MockServer {
    client: Mutex<reqwest::Client>,
    url_map: Mutex<HashMap<String, String>>,
}

#[allow(dead_code)]
pub(crate) struct ServerOpts {
    pub host: &'static str,
    pub port: u16,
}

impl MockServer {
    pub fn new_with_opts(_opts: ServerOpts) -> Self {
        Self {
            client: Mutex::new(Client::new()),
            url_map: Mutex::new(HashMap::new()),
        }
    }

    pub fn url(&self, url: String) -> String {
        self.url_map
            .lock()
            .unwrap()
            .get(&url)
            .cloned()
            .unwrap_or(url)
    }

    pub async fn mock(
        &self,
        method: &str,
        path: &str,
        body: &str,
        status_code: Option<usize>,
        content_type: Option<&str>,
        url: Option<&str>,
    ) -> String {
        let swc = window().unwrap().navigator().service_worker();
        let registration: ServiceWorkerRegistration =
            JsFuture::from(swc.register("/mock_service_worker.js"))
                .await
                .unwrap()
                .unchecked_into();
        JsFuture::from(swc.ready().unwrap()).await.unwrap();
        let sw = registration.active().unwrap();
        let mut nonce = [0; 16];
        getrandom::getrandom(&mut nonce).unwrap();
        let nonce = hex::encode(nonce);
        let config = MockConfig {
            kind: "config".into(),
            method: method.into(),
            path: path.into(),
            status_code: status_code.unwrap_or(200) as u16,
            body: body.into(),
            content_type: content_type.map(|c| c.into()),
        };
        if sw.state() == ServiceWorkerState::Activating {
            JsFuture::from(Promise::new(&mut |rs, _| sw.set_onstatechange(Some(&rs))))
                .await
                .unwrap();
        }
        let client = self.client.lock().unwrap();
        client
            .post(&format!("http://mock_configure/{nonce}"))
            .json(&config)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
        let mock_url = format!("http://mock_{}/", nonce);
        let mocked_url = url
            .unwrap_or(&format!("https://localhost{path}"))
            .to_string();
        let mut url_map = self.url_map.lock().unwrap();
        url_map.insert(mocked_url, mock_url.clone());
        mock_url
    }
}

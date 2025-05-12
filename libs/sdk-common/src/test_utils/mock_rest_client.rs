use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};

use crate::{error::ServiceConnectivityError, prelude::RestClient};

#[derive(Debug)]
pub struct MockResponse {
    pub(crate) status_code: u16,
    pub(crate) text: String,
}

impl MockResponse {
    pub fn new(status_code: u16, text: String) -> Self {
        MockResponse { status_code, text }
    }
}

#[derive(Default)]
pub struct MockRestClient {
    responses: Mutex<VecDeque<MockResponse>>,
}

impl MockRestClient {
    pub fn new() -> Self {
        MockRestClient::default()
    }

    pub fn add_response(&self, response: MockResponse) -> &Self {
        println!("Push response: {response:?}");
        let mut responses = self.responses.lock().unwrap();
        responses.push_back(response);
        self
    }
}

#[sdk_macros::async_trait]
impl RestClient for MockRestClient {
    async fn get(&self, _url: &str) -> Result<(String, u16), ServiceConnectivityError> {
        let mut responses = self.responses.lock().unwrap();
        let response = responses.pop_front().unwrap();
        println!("Pop GET response: {response:?}");
        let status = response.status_code;
        let raw_body = response.text;

        Ok((raw_body, status))
    }

    async fn post(
        &self,
        _url: &str,
        _headers: Option<HashMap<String, String>>,
        _body: Option<String>,
    ) -> Result<(String, u16), ServiceConnectivityError> {
        let mut responses = self.responses.lock().unwrap();
        let response = responses.pop_front().unwrap();
        println!("Pop POST response: {response:?}");
        let status = response.status_code;
        let raw_body = response.text;

        Ok((raw_body, status))
    }
}

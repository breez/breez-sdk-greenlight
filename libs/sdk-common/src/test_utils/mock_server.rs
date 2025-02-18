use std::sync::Mutex;

pub(crate) struct MockServer {
    server: Mutex<mockito::Server>,
}

pub(crate) struct ServerOpts {
    pub host: &'static str,
    pub port: u16,
}

impl MockServer {
    pub fn new_with_opts(opts: ServerOpts) -> Self {
        Self {
            server: Mutex::new(mockito::Server::new_with_opts(mockito::ServerOpts {
                host: opts.host,
                port: opts.port,
                ..Default::default()
            })),
        }
    }

    pub fn url(&self, _url: String) -> String {
        self.server.lock().unwrap().url()
    }

    pub async fn mock(
        &self,
        method: &str,
        path: &str,
        body: &str,
        status_code: Option<usize>,
        content_type: Option<&str>,
        _url: Option<&str>,
    ) -> String {
        let mut server = self.server.lock().unwrap();
        let mut mock = server.mock(method, path).with_body(body);
        if let Some(status_code) = status_code {
            mock = mock.with_status(status_code);
        }
        if let Some(content_type) = content_type {
            mock = mock.with_header("Content-Type", content_type);
        }
        mock.create();
        server.url()
    }
}

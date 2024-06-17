pub mod pay;
mod withdraw;

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use mockito::Server;
    use once_cell::sync::Lazy;

    pub(crate) static MOCK_HTTP_SERVER: Lazy<Mutex<Server>> = Lazy::new(|| {
        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 8080,
            ..Default::default()
        };
        let server = Server::new_with_opts(opts);
        Mutex::new(server)
    });
}

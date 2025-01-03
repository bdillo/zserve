use bytes::Bytes;
use core::fmt;
use std::net::SocketAddr;

use crate::{crypto::generate_tls_config, http::setup_app, mdns::run_mdns};

type Result<T> = std::result::Result<T, ZserveError>;

#[derive(Debug)]
pub enum ZserveError {
    MdnsError(String),
    FileError(String),
    CertificateError(String),
}

impl From<mdns_sd::Error> for ZserveError {
    fn from(value: mdns_sd::Error) -> Self {
        Self::MdnsError(value.to_string())
    }
}

impl From<std::io::Error> for ZserveError {
    fn from(value: std::io::Error) -> Self {
        Self::FileError(value.to_string())
    }
}

impl From<rcgen::Error> for ZserveError {
    fn from(value: rcgen::Error) -> Self {
        Self::CertificateError(value.to_string())
    }
}

impl fmt::Display for ZserveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ZserveError::MdnsError(s) => s,
                ZserveError::FileError(s) => s,
                ZserveError::CertificateError(s) => s,
            }
        )
    }
}

impl std::error::Error for ZserveError {}

pub struct Server {
    name: String,
    port: u16,
}

impl Server {
    pub fn new(name: &str, port: u16) -> Self {
        Self {
            name: name.to_owned(),
            port,
        }
    }

    pub async fn run(&self, file: Bytes) -> Result<()> {
        // this just starts a background thread
        let _mdns = run_mdns(&self.name, self.port)?;

        let app = setup_app(file)?;

        let socket_addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let tls_conf = generate_tls_config().await?;

        axum_server::bind_rustls(socket_addr, tls_conf)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

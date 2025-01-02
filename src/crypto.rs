use axum_server::tls_rustls::RustlsConfig;

use crate::server::ZserveError;

fn generate_cert() -> Result<rcgen::CertifiedKey, rcgen::Error> {
    // TODO: alt names?
    rcgen::generate_simple_self_signed(vec![])
}

pub(crate) async fn generate_tls_config() -> Result<RustlsConfig, ZserveError> {
    let cert_key = generate_cert()?;

    let cert_der = cert_key.cert.pem();
    let key_der = cert_key.key_pair.serialize_pem();

    Ok(RustlsConfig::from_pem(cert_der.into(), key_der.into()).await?)
}

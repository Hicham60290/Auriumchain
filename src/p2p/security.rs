use std::sync::Arc;
use tokio_rustls::{TlsAcceptor, TlsConnector};
use rustls::{Certificate, PrivateKey, ServerConfig, ClientConfig, RootCertStore};
use rcgen::{Certificate as RcgenCert, CertificateParams, DistinguishedName};

pub struct NetworkSecurity {
    pub tls_acceptor: TlsAcceptor,
    pub tls_connector: TlsConnector,
}

impl NetworkSecurity {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Générer certificat auto-signé pour P2P
        let mut params = CertificateParams::new(vec!["auriumchain-node".to_string()]);
        params.distinguished_name = DistinguishedName::new();
        
        let cert = RcgenCert::from_params(params)?;
        let cert_der = cert.serialize_der()?;
        let key_der = cert.serialize_private_key_der();
        
        // Configuration serveur
        let cert_chain = vec![Certificate(cert_der.clone())];
        let private_key = PrivateKey(key_der);
        
        let server_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, private_key)?;
        
        // Configuration client - accepte le certificat auto-signé
        let mut root_store = RootCertStore::empty();
        root_store.add(&Certificate(cert_der))?;
        
        let client_config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        
        Ok(Self {
            tls_acceptor: TlsAcceptor::from(Arc::new(server_config)),
            tls_connector: TlsConnector::from(Arc::new(client_config)),
        })
    }
}

use eyre::*;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpStream;
use tokio_rustls::{server::TlsStream, TlsAcceptor};

pub trait ConnectionListener: Send + Sync + Unpin {
    type Channel1: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static;
    type Channel2: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static;
    type Channel1Future<'a>: Future<Output = Result<(Self::Channel1, SocketAddr)>> + Send + 'a
    where
        Self: 'a;
    type Channel2Future<'a>: Future<Output = Result<Self::Channel2>> + Send + 'a
    where
        Self: 'a;
    fn accept(&self) -> Self::Channel1Future<'_>;
    fn handshake(&self, channel: Self::Channel1) -> Self::Channel2Future<'_>;
}

pub struct TcpListener {
    listener: tokio::net::TcpListener,
}
impl TcpListener {
    pub async fn bind(addr: SocketAddr) -> Result<Self> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }
}
impl ConnectionListener for TcpListener {
    type Channel1 = TcpStream;
    type Channel2 = TcpStream;
    type Channel1Future<'a> = impl Future<Output = Result<(Self::Channel1, SocketAddr)>> + 'a;
    type Channel2Future<'a> = impl Future<Output = Result<Self::Channel2>> + 'a;
    fn accept(&self) -> Self::Channel1Future<'_> {
        async {
            let (stream, addr) = self.listener.accept().await?;
            Ok((stream, addr))
        }
    }
    fn handshake(&self, channel: Self::Channel1) -> Self::Channel2Future<'_> {
        async move { Ok(channel) }
    }
}

pub struct TlsListener<T> {
    tcp: T,
    acceptor: TlsAcceptor,
}
impl<T: ConnectionListener> TlsListener<T> {
    pub async fn bind(under: T, pub_certs: Vec<String>, priv_cert: String) -> Result<Self> {
        let certs = load_certs(&pub_certs)?;
        ensure!(
            !certs.is_empty(),
            "No certificates found in file: {:?}",
            pub_certs
        );
        let keys = load_private_key(&priv_cert)?;
        ensure!(
            !keys.is_empty(),
            "No private key found in file: {}",
            priv_cert
        );
        let key = keys.into_iter().next().context("No private key found")?;

        let tls_cfg = {
            let cfg = rustls::ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(certs, key)?;
            Arc::new(cfg)
        };
        let acceptor = TlsAcceptor::from(tls_cfg);
        Ok(Self {
            tcp: under,
            acceptor,
        })
    }
}
impl<T: ConnectionListener + 'static> ConnectionListener for TlsListener<T> {
    type Channel1 = T::Channel1;
    type Channel2 = TlsStream<T::Channel2>;
    type Channel1Future<'a> = impl Future<Output = Result<(Self::Channel1, SocketAddr)>> + 'a;
    type Channel2Future<'a> = impl Future<Output = Result<Self::Channel2>> + 'a;
    fn accept(&self) -> Self::Channel1Future<'_> {
        self.tcp.accept()
    }
    fn handshake(&self, channel: Self::Channel1) -> Self::Channel2Future<'_> {
        async {
            let channel = self.tcp.handshake(channel).await?;
            let tls_stream = self.acceptor.accept(channel).await?;
            Ok(tls_stream)
        }
    }
}

// Load public certificate from file.
pub fn load_certs<T: AsRef<str>>(
    path: impl IntoIterator<Item = T>,
) -> Result<Vec<rustls::Certificate>> {
    let mut r_certs = vec![];
    for p in path {
        let p = p.as_ref();
        let f = std::fs::File::open(p).with_context(|| format!("Failed to open {}", p))?;
        let certs = certs(&mut std::io::BufReader::new(f))
            .with_context(|| format!("Invalid certificate {}", p))?
            .into_iter()
            .map(rustls::Certificate);
        r_certs.extend(certs)
    }
    Ok(r_certs)
}

// Load private key from file.
pub fn load_private_key(path: &str) -> Result<Vec<rustls::PrivateKey>> {
    pkcs8_private_keys(&mut std::io::BufReader::new(std::fs::File::open(path)?))
        .with_context(|| format!("Invalid private key {}", path))
        .map(|mut keys| keys.drain(..).map(rustls::PrivateKey).collect())
}

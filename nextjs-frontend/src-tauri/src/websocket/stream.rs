use super::*;

use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;

pub(super) async fn websocket_stream(
    ws_url: &str,
) -> Result<tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let request = url::Url::parse(ws_url).unwrap();
    let (socket, _) = tokio_tungstenite::client_async_tls_with_config(
        request.clone(),
        tcp_stream(request)?,
        None,
        Some(tokio_tungstenite::Connector::Rustls(config())),
    )
    .await
    .map_err(|e| anyhow::anyhow!(e))?;
    Ok(socket)
}

fn tcp_stream(request: url::Url) -> Result<TcpStream> {
    use rustls::ClientConfig;
    use tungstenite::client::IntoClientRequest as _;

    let request = request.into_client_request().unwrap();
    let uri = request.uri();
    let mode =
        tungstenite::client::uri_mode(uri).map_err(|e| error::Error::System(e.to_string()))?;
    let host = uri.host().ok_or_else(|| {
        error::Error::System(tungstenite::error::UrlError::NoHostName.to_string())
    })?;
    let port = uri.port_u16().unwrap_or(match mode {
        tungstenite::stream::Mode::Plain => 80,
        tungstenite::stream::Mode::Tls => 443,
    });

    use std::net::ToSocketAddrs as _;
    let addrs = (host, port).to_socket_addrs().map_err(|e| {
        error!("to socket addr connect error: {e}");
        error::Error::System(e.to_string())
    })?;
    let stream = connect_to_some(addrs.as_slice(), uri).map_err(|e| {
        error!("TcpStream connect error: {e}");
        error::Error::System(e.to_string())
    })?;

    stream
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let tcp_stream = TcpStream::from_std(stream).expect("tokio tcp_stream call failed");
    Ok(tcp_stream)
}

fn config() -> Arc<rustls::ClientConfig> {
    mod danger {
        pub struct NoCertificateVerification {}

        impl rustls::client::ServerCertVerifier for NoCertificateVerification {
            fn verify_server_cert(
                &self,
                _end_entity: &rustls::Certificate,
                _intermediates: &[rustls::Certificate],
                _server_name: &rustls::ServerName,
                _scts: &mut dyn Iterator<Item = &[u8]>,
                _ocsp_response: &[u8],
                _now: instant::SystemTime,
            ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
                Ok(rustls::client::ServerCertVerified::assertion())
            }
        }
    }

    Arc::new(
        rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(danger::NoCertificateVerification {}))
            .with_no_client_auth(),
    )
}

fn connect_to_some(
    addrs: &[std::net::SocketAddr],
    uri: &tungstenite::http::Uri,
) -> Result<std::net::TcpStream, error::Error> {
    for addr in addrs {
        if let Ok(stream) = std::net::TcpStream::connect(addr) {
            return Ok(stream);
        }
    }
    Err(error::Error::System(format!("url error: {uri}")))
}

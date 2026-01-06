use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use dav_server::DavHandler;
use dav_server_opendalfs::OpendalFs;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use opendal::{Operator, Scheme};
use tokio::net::TcpListener;
use tracing::{error, info};

const ENV_PREFIX: &str = "OPENDAL_";

fn build_operator_from_env() -> Result<Operator, opendal::Error> {
    let scheme_str =
        env::var("OPENDAL_TYPE").expect("OPENDAL_TYPE environment variable is required");

    let scheme = Scheme::from_str(&scheme_str)
        .map_err(|e| opendal::Error::new(opendal::ErrorKind::ConfigInvalid, e.to_string()))?;

    let config: HashMap<String, String> = env::vars()
        .filter(|(k, _)| k.starts_with(ENV_PREFIX) && k != "OPENDAL_TYPE")
        .map(|(k, v)| {
            let key = k.strip_prefix(ENV_PREFIX).unwrap().to_lowercase();
            (key, v)
        })
        .collect();

    info!(scheme = %scheme, config_keys = ?config.keys().collect::<Vec<_>>(), "Building operator");

    Operator::via_iter(scheme, config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let op = build_operator_from_env()?;
    let webdav_fs = OpendalFs::new(op);

    let dav_handler = DavHandler::builder().filesystem(webdav_fs).build_handler();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid u16");

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = TcpListener::bind(addr).await?;

    info!(%addr, "WebDAV server listening");

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let handler = dav_handler.clone();

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| {
                        let handler = handler.clone();
                        async move { Ok::<_, Infallible>(handler.handle(req).await) }
                    }),
                )
                .await
            {
                error!(%remote_addr, %err, "Connection error");
            }
        });
    }
}

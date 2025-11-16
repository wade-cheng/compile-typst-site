// Code is adapted from MIT-licensed https://github.com/leotaku/tower-livereload/tree/master/examples/livehttpd.

use std::{io::ErrorKind, net::SocketAddr, path::PathBuf, sync::mpsc::Receiver, thread};

use anyhow::{Result, anyhow};
use axum::{Router, http};
use tokio::net::TcpListener;
use tower::layer::util::Stack;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use tower_livereload::LiveReloadLayer;

// not sure if this no cache stuff is needed, Works On My Machine, but might as well keep it in.
// I don't notice any terrible performance issues or anything.

type Srhl = SetResponseHeaderLayer<http::HeaderValue>;

fn no_cache_layer() -> Stack<Srhl, Stack<Srhl, Srhl>> {
    Stack::new(
        SetResponseHeaderLayer::overriding(
            http::header::CACHE_CONTROL,
            http::HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        ),
        Stack::new(
            SetResponseHeaderLayer::overriding(
                http::header::PRAGMA,
                http::HeaderValue::from_static("no-cache"),
            ),
            SetResponseHeaderLayer::overriding(
                http::header::EXPIRES,
                http::HeaderValue::from_static("0"),
            ),
        ),
    )
}

pub async fn serve(reload_rx: Receiver<()>, path: PathBuf) {
    if let Err(error) = try_serve(reload_rx, path).await {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}

async fn try_serve(reload_rx: Receiver<()>, path: PathBuf) -> Result<()> {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    // yes, we just use a normal thread instead of tokio shenanigans. meh.
    thread::spawn(move || {
        for _reload_request in reload_rx {
            reloader.reload();
        }
    });

    let app = Router::new()
        .fallback_service(ServeDir::new(path))
        .layer(livereload)
        .layer(no_cache_layer());

    let mut listener: Option<TcpListener> = None;
    let mut addr: Option<SocketAddr> = None;
    let ports = 8000..8050;
    for port in ports.clone() {
        let candidate_addr: std::net::SocketAddr = ([0, 0, 0, 0], port).into();
        match tokio::net::TcpListener::bind(candidate_addr).await {
            Ok(candidate_listener) => {
                listener = Some(candidate_listener);
                addr = Some(candidate_addr);
                break;
            }
            Err(e) => {
                if e.kind() == ErrorKind::AddrInUse {
                    continue;
                } else {
                    return Err(e.into());
                }
            }
        }
    }

    let (Some(listener), Some(addr)) = (listener, addr) else {
        return Err(anyhow!(
            "Couldn't serve your website locally. We tried binding to ports {:?}",
            ports
        ));
    };

    log::info!("serving your website locally at the link: http://{}/", addr);

    // tracing_subscriber::fmt::init(); // uhh apparently someone already called this somewhere?

    axum::serve(listener, app).await?;

    Ok(())
}

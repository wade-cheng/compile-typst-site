use std::{sync::mpsc::Receiver, thread};

use axum::{Router, http};
use tower::layer::util::Stack;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use tower_livereload::LiveReloadLayer;

// added crates: tokio
// axuma xum tower tower_http tower_livereload

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

pub async fn serve(reload_rx: Receiver<()>) {
    if let Err(error) = try_main(reload_rx).await {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

async fn try_main(reload_rx: Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    // yes, we just use a normal thread instead of tokio shenanigans. meh.
    thread::spawn(move || {
        for _reload_request in reload_rx {
            reloader.reload();
        }
    });

    let app = Router::new()
        .fallback_service(ServeDir::new(
            "/home/saffron/Documents/projects/compile-typst-site/examples/typst-site-full/_site",
        ))
        .layer(livereload)
        .layer(no_cache_layer());

    let addr: std::net::SocketAddr = ([0, 0, 0, 0], 8010).into();
    eprintln!("listening on: http://{}/", addr);

    // tracing_subscriber::fmt::init(); // uhh apparently someone already called this somewhere?

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

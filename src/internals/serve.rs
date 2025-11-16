use std::path::Path;

use axum::{Router, http};
use notify_debouncer_full::notify::{self, RecursiveMode, Watcher as _};
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

pub async fn serve() {
    if let Err(error) = try_main().await {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

async fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("tried main");
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .fallback_service(ServeDir::new(
            "/home/saffron/Documents/projects/compile-typst-site/examples/typst-site-full/_site",
        ))
        .layer(livereload)
        .layer(no_cache_layer());

    let mut watcher = notify::recommended_watcher(move |event: Result<_, _>| {
        if event.is_ok_and(|evt: notify::Event| !evt.kind.is_access()) {
            reloader.reload();
        }
    })?;
    watcher.watch(
        Path::new(
            "/home/saffron/Documents/projects/compile-typst-site/examples/typst-site-full/_site",
        ),
        RecursiveMode::Recursive,
    )?;

    let addr: std::net::SocketAddr = ([0, 0, 0, 0], 8010).into();
    eprintln!("listening on: http://{}/", addr);

    // tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

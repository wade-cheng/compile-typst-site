// Code is adapted from MIT-licensed https://github.com/leotaku/tower-livereload/tree/master/examples/livehttpd.

use std::{
    fs,
    io::{BufRead, BufReader, BufWriter, ErrorKind, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    sync::{Arc, Mutex, mpsc::Receiver},
    thread,
};

use anyhow::{Result, anyhow};

const LIVE_RELOAD_SCRIPT: &str = r#"<script>
    const source = new EventSource('/livereload');
    source.onmessage = () => {
        source.close(); 
        location.reload();
    }
    source.onerror = () => {
        source.close();
    };
    window.onbeforeunload = () => {
        source.close();
    };
</script>"#;

fn guess_mime_type(path: &PathBuf) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("pdf") => "application/pdf",
        _ => "application/octet-stream",
    }
}

fn inject_livereload_script(html: &str) -> String {
    // Try to inject before </body>, otherwise before </html>, otherwise at the end
    if let Some(pos) = html.rfind("</body>") {
        format!(
            "{}{}</body>{}",
            &html[..pos],
            LIVE_RELOAD_SCRIPT,
            &html[pos + 7..]
        )
    } else if let Some(pos) = html.rfind("</html>") {
        format!(
            "{}{}</html>{}",
            &html[..pos],
            LIVE_RELOAD_SCRIPT,
            &html[pos + 7..]
        )
    } else {
        format!("{}{}", html, LIVE_RELOAD_SCRIPT)
    }
}

fn write_404_response(mut stream: TcpStream) -> Result<()> {
    stream.write_all(
        b"HTTP/1.1 400 NOT FOUND\r\n\
        Content-Length: 10\r\n\r\n\
        Error 404.",
    )?;

    Ok(())
}

/// Kick off server-side events api.
fn handle_sse(mut stream: TcpStream, hot_reload_clients: Arc<Mutex<Vec<TcpStream>>>) -> Result<()> {
    stream.write_all(
        b"HTTP/1.1 200 OK\r\n\
        Content-Type: text/event-stream\r\n\
        Cache-Control: no-cache\r\n\
        Connection: keep-alive\r\n\r\n",
    )?;

    hot_reload_clients.lock().unwrap().push(stream);

    Ok(())
}

fn handle_connection(
    stream: TcpStream,
    output_path: &PathBuf,
    hot_reload_clients: Arc<Mutex<Vec<TcpStream>>>,
) -> Result<()> {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap_or(Ok("".into()))?;

    // only respond to these specific stuff. keep it lean.
    if let Some(url_path_and_version) = request_line.strip_prefix("GET ")
        && let Some(url_path) = url_path_and_version.strip_suffix(" HTTP/1.1")
    {
        if url_path == "/livereload" {
            handle_sse(stream, hot_reload_clients)?;
            return Ok(());
        }

        // not SSE path, so let's serve static file.

        let mut file_path = output_path.join(url_path.trim_start_matches('/'));

        if file_path.is_dir() {
            file_path = file_path.join("index.html");
        }

        if !file_path.exists() && file_path.extension().is_none() {
            let with_html = file_path.with_extension("html");
            if with_html.exists() {
                file_path = with_html;
            }
        }

        let file = fs::read(&file_path);
        let content = match file {
            Ok(content) => Ok(content),
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    write_404_response(stream)?;
                    return Ok(());
                } else {
                    Err(e)
                }
            }
        }?;

        let mime_type = guess_mime_type(&file_path);

        // Inject live reload script into HTML files
        let response_body = if mime_type.starts_with("text/html") {
            let html = String::from_utf8_lossy(&content);
            let injected = inject_livereload_script(&html);
            injected.into_bytes()
        } else {
            content
        };

        let mut buf_writer = BufWriter::new(stream);
        buf_writer.write_all(b"HTTP/1.1 200 OK\r\n")?;
        buf_writer.write_all(b"Content-Type: ")?;
        buf_writer.write_all(mime_type.as_bytes())?;
        buf_writer.write_all(b"\r\n")?;
        buf_writer.write_all(b"Cache-Control: no-cache, no-store, must-revalidate\r\n")?;
        buf_writer.write_all(b"Pragma: no-cache\r\n")?;
        buf_writer.write_all(b"Expires: 0\r\n")?;
        buf_writer.write_all(b"\r\n")?;
        buf_writer.write_all(&response_body)?;
        buf_writer.flush()?;
    } else {
        write_404_response(stream)?;
    }
    Ok(())
}

fn bind() -> Result<TcpListener> {
    let mut listener: Option<TcpListener> = None;
    let mut addr: Option<SocketAddr> = None;
    let ports = 8000..8050;
    for port in ports.clone() {
        let candidate_addr: std::net::SocketAddr = ([0, 0, 0, 0], port).into();
        match TcpListener::bind(candidate_addr) {
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

    Ok(listener)
}

pub fn serve(reload_rx: Receiver<()>, path: PathBuf) -> Result<()> {
    let hot_reload_clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

    let hrc = hot_reload_clients.clone();
    thread::spawn(move || {
        for _reload_request in reload_rx {
            let mut streams = hrc.lock().unwrap();
            streams.retain(|mut tcp_stream| tcp_stream.write_all(b"data: reload\r\n\r\n").is_ok());
            log::debug!("Tracking {} stream(s) for hot reloading.", streams.len());
        }
    });

    let listener = bind()?;

    thread::scope(|s| {
        for stream in listener.incoming() {
            s.spawn(|| {
                let stream = stream.unwrap();

                handle_connection(stream, &path, hot_reload_clients.clone()).unwrap();
            });
        }
    });

    Ok(())
}

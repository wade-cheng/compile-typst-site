// Code is adapted from MIT-licensed https://github.com/leotaku/tower-livereload/tree/master/examples/livehttpd.

use std::{
    io::{BufRead, BufReader, ErrorKind, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    sync::{Arc, Mutex, mpsc::Receiver},
    thread,
};

use anyhow::{Result, anyhow};

fn handle_connection(stream: TcpStream) -> Result<()> {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap_or(Ok("".into()))?;
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
            hrc.lock().unwrap().retain(|mut tcp_stream| {
                tcp_stream.write_all(b"data: reload\r\n\r\n").unwrap();
                true
            });
        }
    });

    let listener = bind()?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)?;
    }

    Ok(())
}

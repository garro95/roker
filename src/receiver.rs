use tokio::{net::{TcpListener, TcpStream}, spawn};
use log::{info, error, warn};
use tokio_tungstenite::{accept_async, tungstenite::Error};

use std::net::SocketAddr;

use crate::connections::Connection;

async fn listen(listener: TcpListener) {
    loop {
	match listener.accept().await {
	    Ok((stream, _)) => {
		match stream.peer_addr() {
		    Ok(peer) => {
			info!("Peer address: {}", peer);
			
			spawn(accept_connection(peer, stream));
		    },
		    Err(e) => {
			error!("Connected streams should have a peer address. {}", e);
			stream.close();
		    }
		}
	    },
	    Err(e) => {
		error!("{}", e);
	    }
	}
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;

    info!("New WebSocket connection: {}", peer);

    Connection::new(stream).await;

    Ok(())
}

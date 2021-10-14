/*
 *  Copyright 2021 Gianmarco Garrisi
 *
 *  This file is part of roker.
 *
 *  roker is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  roker is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with roker.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

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

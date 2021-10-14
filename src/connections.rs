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

use tungstenite::protocol::Message;
use tokio::time::Interval;
use futures::{sink::Sink, Stream};

use crate::stomp::{StompCommand, StompFrame};

use std::sync::atomic::AtomicBool;

pub struct Connection<T>
where T: Sink + Stream
{
    channel: T,
    heartbeat_received: AtomicBool,
    client_heartbeat: Option<Interval>,
    server_heartbeat: Option<Interval>,
}

impl<T> Connection<T>
where T: Sink + Stream {
    pub async fn new(channel: T) -> Connection<T> {
	
	while let Some(msg) = channel.next().await {
            let msg = msg?;

	    let frame = if let Message::Text(m) = msg {
		let lines = m.lines();

		// First line is the command
		let command: StompCommand = lines.next().try_into()?;

		let mut frame = StompFrame::new(command);

		// Followed by headers
		while let line = lines.next() {
		    if line.is_empty() {
			// end of headers
			break;
		    }

		    let (header, value) = line.split_once(":");
		    frame.push_header(header.into(), value.into());
		}

		// And optionally by a body
		for line in lines {
		    frame.push_body(line);
		}

		frame
	    };
	}
    }
}

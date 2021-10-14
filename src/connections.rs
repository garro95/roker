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

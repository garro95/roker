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

use std::convert::TryFrom;

pub enum StompError {
    UnknownCommand
}

#[derive(Copy, Clone)]
pub enum StompCommand {
    Connect,
    Stomp,
    Connected,
    Error,
    Send,
    Subscribe,
    Unsubscribe,
    Begin,
    Commit,
    Abort,
    Ack,
    Nack,
    Disconnect,
    Message,
    Receipt
}

impl StompCommand {
    pub fn as_str(&self) -> &str {
        match self {
            StompCommand::Ack => "ACK",
            StompCommand::Send => "SEND",
            StompCommand::Nack => "NACK",
            StompCommand::Begin => "BEGIN",
            StompCommand::Abort => "ABORT",
            StompCommand::Error => "ERROR",
            StompCommand::Stomp => "STOMP",
            StompCommand::Commit => "COMMIT",
            StompCommand::Connect => "CONNECT",
            StompCommand::Message => "MESSAGE",
            StompCommand::Receipt => "RECEIPT",
            StompCommand::Subscribe => "SUBSCRIBE",
            StompCommand::Connected => "CONNECTED",
            StompCommand::Disconnect => "DISCONNECT",
            StompCommand::Unsubscribe => "UNSUBSCRIBE"
        }
    }
}

impl TryFrom<&str> for StompCommand {
    type Error = StompError;
    fn try_from(value: &str) -> Result<Self, StompError> {
	match value {
	    "ACK" => Ok(Self::Ack),
	    "SEND" => Ok(Self::Send),
	    "NACK" => Ok(Self::Nack),
	    "BEGIN" => Ok(Self::Begin),
	    "ABORT" => Ok(Self::Abort),
	    "ERROR" => Ok(Self::Error),
	    "STOMP" => Ok(Self::Stomp),
	    "COMMIT" => Ok(Self::Commit),
	    "CONNECT" => Ok(Self::Connect),
	    "MESSAGE" => Ok(Self::Message),
	    "RECEIPT" => Ok(Self::Receipt),
	    "SUBSCRIBE" => Ok(Self::Subscribe),
	    "CONNECTED" => Ok(Self::Connected),
	    "DISCONNECT" => Ok(Self::Disconnect),
	    "UNSUBSCRIBE" => Ok(Self::Unsubscribe),
	    _ => Err(StompError::UnknownCommand),
	}
    }
}

pub struct StompFrame {
    command: StompCommand,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl StompFrame {
    pub fn new(command: StompCommand) -> StompFrame {
	StompFrame {
	    command,
	    headers: vec![],
	    body: None
	}
    }

    pub fn headers(&self) -> &[(String, String)] {
	self.headers.as_slice()
    }

    pub fn push_header(&mut self, key: String, value: String) {
	self.headers.push((key, value));
    }

    pub fn set_body(&mut self, body: String) -> Option<String> {
	self.body.replace(body)
    }

    pub fn body(&self) -> Option<&str> {
	self.body.map(|s| s.as_str())
    }

    pub fn push_body(&mut self, str: String) {
	self.body = match self.body.take() {
	    None => Some(str),
	    Some(body) => Some(body + "\n" + str.as_str())
	};
    }
}

impl From<StompError> for StompFrame {
    fn from(error: StompError) -> StompFrame {
	let frame = StompFrame::new(StompCommand::Error);

	match error {
	    UnknownCommand => frame.push_header("message".into(), "Unknown command received".into()),
	}

	frame
    }
}

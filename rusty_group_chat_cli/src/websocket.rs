use serde::{de::DeserializeOwned, Serialize};
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

// TODO: Use WebsocketError interface for all errors
pub struct WebsocketError;
pub struct Websocket {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl Websocket {
    pub fn connect(url: &str, error_message: &str) -> Self {
        let (socket, _connect_response) = connect(url).expect(error_message);

        Websocket { socket }
    }

    pub fn send_json(&mut self, json: &impl Serialize) -> Result<(), tungstenite::Error> {
        self.socket
            .write_message(Message::Text(serde_json::to_string(json).unwrap()))
    }

    pub fn read_json<T: DeserializeOwned>(&mut self) -> Result<T, String> {
        match self.socket.read_message() {
            Ok(Message::Text(json_text)) => Ok(serde_json::from_str(&json_text).unwrap()),
            err => Err(format!("Expected JSON. Got: {}", err.unwrap())),
        }
    }
}

///////////////////////////////////
//     WEBSOCKET_URL BOUNDARY   //
/////////////////////////////////
pub struct WebsocketURL {
    url: String,
}

impl WebsocketURL {
    pub fn new(url: String) -> Self {
        if !Self::is_ws_prefixed(&url) {
            WebsocketURL {
                url: Self::wss().to_owned() + &url,
            }
        } else if Self::is_http_prefixed(&url) {
            WebsocketURL {
                url: url
                    .replace(Self::http(), Self::ws())
                    .replace(Self::https(), Self::wss()),
            }
        } else {
            WebsocketURL { url }
        }
    }

    pub fn value(&self) -> &str {
        &self.url
    }

    fn is_ws_prefixed(url: &str) -> bool {
        url.starts_with(Self::ws()) || url.starts_with(Self::wss())
    }

    fn is_http_prefixed(url: &str) -> bool {
        url.starts_with(Self::http()) || url.starts_with(Self::https())
    }

    fn http() -> &'static str {
        "http://"
    }

    fn https() -> &'static str {
        "https://"
    }

    fn ws() -> &'static str {
        "ws://"
    }
    fn wss() -> &'static str {
        "wss://"
    }
}

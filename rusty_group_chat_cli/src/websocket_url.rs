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

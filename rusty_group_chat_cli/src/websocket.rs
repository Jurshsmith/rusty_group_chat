use url::Url;
#[derive(Debug)]
pub struct WebsocketURL {
    url: String,
}

impl WebsocketURL {
    pub fn new(url: String) -> Self {
        if Self::is_http_prefixed(&url) {
            WebsocketURL {
                url: url
                    .replace(Self::http(), Self::ws())
                    .replace(Self::https(), Self::wss()),
            }
            .sanitize()
        } else if !Self::is_ws_prefixed(&url) {
            WebsocketURL {
                url: Self::wss().to_owned() + &url,
            }
            .sanitize()
        } else {
            WebsocketURL { url }.sanitize()
        }
    }

    pub fn append_query_param(&self, key: &str, value: &str) -> Self {
        WebsocketURL {
            url: Url::parse_with_params(&self.url, [(key, value)])
                .unwrap()
                .to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.url
    }

    fn sanitize(&self) -> Self {
        WebsocketURL {
            url: self.url.trim().to_owned(),
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

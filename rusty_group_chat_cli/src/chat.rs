use crate::{Terminal, Websocket, WebsocketURL};

pub struct GroupChatDetails {
    url: WebsocketURL,
    username: String,
}

impl GroupChatDetails {
    pub fn collect() -> Self {
        println!("Enter the group chat URL:");

        let url = Terminal::read_line().unwrap();

        println!("Enter a cool alias others can identify you with:");

        let username = Terminal::read_line().unwrap();

        GroupChatDetails {
            url: WebsocketURL::new(url),
            username,
        }
    }

    pub fn url(&self) -> String {
        self.url.value().to_owned()
    }
}

///////////////////////////////////
//     GROUP_CHAT BOUNDARY      //
/////////////////////////////////

pub struct GroupChat {
    socket: Websocket,
}

impl GroupChat {
    pub fn join_with(details: GroupChatDetails) -> Self {
        let socket = Websocket::connect(
            &details.url(),
            "Couldn't connect to the provided url. Please check if the url is valid and try again.",
        );

        GroupChat { socket }
    }
}

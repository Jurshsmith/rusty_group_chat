use crate::{Terminal, WebsocketURL};

const INFO: &str = r#"

Welcome To Rusty Group Chat 🦀

 - Press 'Esc' to quit

"#;

#[derive(Debug)]
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
            url: WebsocketURL::new(url).append_query_param("name", &username),
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
use std::env;

use futures_channel::mpsc::UnboundedSender;
use futures_util::{future, pin_mut, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct GroupChat {
    // socket: Websocket,
}

impl GroupChat {
    pub fn init() {
        println!("{}", INFO);
    }

    pub async fn join_with(details: GroupChatDetails) -> Self {
        let (socket_sink, socket_stream) = futures_channel::mpsc::unbounded();
        // tokio::spawn(read_stdin(stdin_tx));

        tokio::spawn(async move {
            read_and_send_chat(socket_sink);
        });
        // tokio::spawn(async move {
        //     let chat = Terminal::read_line().unwrap();

        //     socket_sink.unbounded_send(Message::Text(chat)).unwrap();
        // });

        let (ws_stream, _) = connect_async(details.url())
            .await
            .expect("Failed to connect");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = socket_stream.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                match message {
                    Ok(message) => {
                        let received_message = message.into_text().unwrap();
                        // TODO: Pattern match if this a system message that user already exists ?
                        // And Redo Cool Alias Prompt!
                        println!("{}", &received_message);
                    }
                    Err(_error) => {}
                }
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;

        GroupChat {}
    }
}

fn read_and_send_chat(socket_sink: UnboundedSender<Message>) {
    while let Ok(chat) = Terminal::read_line() {
        socket_sink.unbounded_send(Message::Text(chat)).unwrap();
    }
}

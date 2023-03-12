use crate::{Terminal, WebsocketURL};
use rusty_group_chat::{Chat, ChatSender, User};

const INFO: &str = r#"

Welcome To Rusty Group Chat 🦀

 - Press 'Enter' to send chat/prompt
 - Press 'Esc' to quit

"#;

pub struct GroupChatDetails {
    url: WebsocketURL,
    user: User,
}

impl GroupChatDetails {
    pub fn collect() -> Self {
        println!("Enter the group chat URL:");

        let url = Terminal::read_line().unwrap();

        println!("Enter a cool alias others can identify you with:");

        let username = Terminal::read_line().unwrap().trim().to_string();

        GroupChatDetails {
            url: WebsocketURL::new(url).append_query_param("name", &username),
            user: User::from_name(username),
        }
    }

    pub fn url(&self) -> String {
        self.url.value().to_owned()
    }
}

///////////////////////////////////
//     GROUP_CHAT BOUNDARY      //
/////////////////////////////////
use futures_util::{future, pin_mut, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct GroupChat;

impl GroupChat {
    pub fn init() {
        println!("{}", INFO);
    }

    // TODO: Break down this function
    pub async fn join_with(details: GroupChatDetails) -> Self {
        let (socket_sink, socket_stream) = futures_channel::mpsc::unbounded();

        tokio::spawn(async move {
            while let Ok(chat) = Terminal::read_line() {
                if !chat.is_empty() {
                    socket_sink.unbounded_send(Message::Text(chat)).unwrap();
                    Terminal::clear_last_line();
                }
            }
        });

        let (ws_stream, _) = connect_async(details.url())
            .await
            .expect("Failed to connect");

        let (write, chats_stream) = ws_stream.split();

        let stdin_to_ws = socket_stream.map(Ok).forward(write);
        let ws_to_stdout = {
            chats_stream.for_each(|chat| async {
                match chat {
                    Ok(chat) => {
                        let received_chat = Chat::from_string(&chat.into_text().unwrap());

                        match received_chat.from {
                            ChatSender::System => {
                                // TODO: Remove this hack and make chat more structured
                                if received_chat.message.contains(&details.user.name) {
                                    println!(
                                        "\n{}",
                                        &received_chat.message.replace(&details.user.name, "You")
                                    );
                                    println!(
                                        "Bravo! You can now type and send your chat messages.\n"
                                    )
                                } else {
                                    println!("{}", &received_chat.message)
                                }
                            }

                            ChatSender::User(user) => {
                                if user.is_equal_to(&details.user) {
                                    println!("You: {}\n", &received_chat.message);
                                } else {
                                    println!("{}: {}\n", &user.name, &received_chat.message);
                                }
                            }
                        }
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

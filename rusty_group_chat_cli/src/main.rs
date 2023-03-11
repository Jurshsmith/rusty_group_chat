use rusty_group_chat_cli::{GroupChat, GroupChatDetails};

#[tokio::main]
async fn main() -> Result<(), ()> {
    GroupChat::init();

    let group_chat_details = GroupChatDetails::collect();

    GroupChat::join_with(group_chat_details).await;

    Ok(())
}

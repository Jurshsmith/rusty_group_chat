use rusty_group_chat_cli::{GroupChat, GroupChatDetails, Terminal};

fn main() -> Result<(), ()> {
    Terminal::init();

    let group_chat_details = GroupChatDetails::collect();

    GroupChat::join_with(group_chat_details);

    Ok(())
}

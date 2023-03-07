use rusty_group_chat_cli::{ChatSessionDetails, Terminal};

fn main() -> Result<(), ()> {
    Terminal::init();

    ChatSessionDetails::collect();

    Ok(())
}

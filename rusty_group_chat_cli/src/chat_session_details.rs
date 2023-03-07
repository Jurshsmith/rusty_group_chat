use crate::Terminal;

pub struct ChatSessionDetails;

impl ChatSessionDetails {
    pub fn collect() {
        println!("Enter the group chat URL:");

        println!("{:?}", Terminal::read_line());

        println!("Enter a cool alias others can identify you with:");

        println!("{:?}", Terminal::read_line());
    }
}

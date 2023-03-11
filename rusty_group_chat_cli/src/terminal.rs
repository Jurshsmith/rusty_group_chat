pub struct Terminal;

impl Terminal {
    pub fn read_line() -> Result<String, io::Error> {
        CrosstermTerminal::read_line()
    }
}

///////////////////////////////////
//      CROSSTERM BOUNDARY      //
/////////////////////////////////
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

struct CrosstermTerminal;

// Helps Abstract CrossTerm functions to provide an higher level generic API
impl CrosstermTerminal {
    pub fn read_line() -> Result<String, io::Error> {
        let mut line = String::new();
        while let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char(c) => {
                    line.push(c);
                }
                _ => {}
            }
        }

        Ok(line)
    }
}

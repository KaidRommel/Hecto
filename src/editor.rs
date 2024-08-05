use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminals;
use terminals::Terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Location {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool
}

impl Editor {
    pub const fn default() -> Self {
        Editor{
            should_quit: false
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(
            KeyEvent{
                code: Char('q'),
                modifiers: KeyModifiers::CONTROL,
                .. // 忽略其他字段
            }
        ) = event {
            self.should_quit = true;
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clean_screen()?;
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.0 as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message)
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")
    }
    
    fn draw_rows() -> Result<(), std::io::Error> {
        // 获取终端行数
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}

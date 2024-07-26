use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminals;
use terminals::Terminal;

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
        if self.should_quit {
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        // 获取终端行数
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }

}

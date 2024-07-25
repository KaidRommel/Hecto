use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use std::io::stdout;

pub struct Editor {
    should_quit: bool
}

impl Editor {
    pub fn default() -> Self {
        Editor{
            should_quit: false
        }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    // 初始化
    fn initialize() -> Result<(), std::io::Error>{
        enable_raw_mode()?;
        Self::clean_screen()
    }

    // 结束
    fn terminate() -> Result<(), std::io::Error>{
        disable_raw_mode()
    }

    // 清屏
    fn clean_screen() -> Result<(), std::io::Error>{
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            if self.should_quit {
                break;
            }
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
}

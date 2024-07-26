use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::execute;
use crossterm::cursor::{MoveTo};
use std::io::stdout;

pub struct Terminal{}

impl Terminal {
    // 初始化
    pub fn initialize() -> Result<(), std::io::Error>{
        enable_raw_mode()?;
        Self::clean_screen()
    }

    // 结束
    pub fn terminate() -> Result<(), std::io::Error>{
        disable_raw_mode()
    }

    // 清屏
    pub fn clean_screen() -> Result<(), std::io::Error>{
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
        //execute!(stdout, MoveTo(0,0))
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error>{
        execute!(stdout(), MoveTo(x, y))
    }

    // 获取窗口的列数和行数
    pub fn size() -> Result<(u16, u16), std::io::Error>{
        size()
    }
}
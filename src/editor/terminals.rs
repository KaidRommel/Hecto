use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{execute, queue};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use std::io::{stdout, Write};
use core::fmt::Display;

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
        queue!(stdout(), Clear(ClearType::All))
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error>{
        queue!(stdout(), MoveTo(x, y))
    }

    pub fn show_cursor() -> Result<(), std::io::Error>{
        queue!(stdout(), Show)
    }

    pub fn hide_cursor() -> Result<(), std::io::Error>{
        queue!(stdout(), Hide)
    }

    pub fn print(message: impl Display) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(message))
    }
    
    // 获取窗口的列数和行数
    pub fn size() -> Result<(u16,u16), std::io::Error>{
        size()
    }
    
    // 执行缓冲区的指令
    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

    // 将指令载入缓冲区
    // todo

    

    
}
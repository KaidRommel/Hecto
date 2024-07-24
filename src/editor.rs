use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{read, Event::Key, KeyCode::Char};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor{}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
    }

    pub fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?}\r",event);
                    match event.code {
                        Char(char) => {
                            if char == 'q'{
                                break;
                            }
                        }
                        _ => ()
                    }
                },
                Err(err) => {
                    println!("{} \r", err)
                }
                _ => ()
            }
        }
        disable_raw_mode()?;
        Ok(())
    }

}

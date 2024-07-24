use std::io::{stdin, stdout, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


fn main() {
    enable_raw_mode().unwrap();
    for input in stdin().bytes() {
        let input = input.unwrap();
        let char = input as char;
        if char == 'q' {
            disable_raw_mode().unwrap();
            return;
        } else if char.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r",input);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r",input, char)
        }
    }
}

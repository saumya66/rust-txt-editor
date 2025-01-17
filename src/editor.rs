use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

//pub means public
pub struct Editor {
    //equivalent of class in other langs
}

impl Editor {
    //Self is the return type here, as this public function
    //will help create and return an instance of struct Editor
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;
                    if c.is_control() {
                        println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
                    } else {
                        println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                    }
                    if c == 'q' {
                        disable_raw_mode().unwrap();
                        break;
                    }
                }
                Err(error) => println!("Error: {}", error),
            }
        }
    }
}

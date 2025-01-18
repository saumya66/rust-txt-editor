use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

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
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    match event.code {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                Err(error) => println!("Error: {}", error),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}

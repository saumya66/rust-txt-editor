use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

//pub means public
pub struct Editor {
    //equivalent of class in other langs
}

impl Editor {
    //Self is the return type here, as this public function
    //will help create and return an instance of struct Editor
    pub fn new() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            //We have done this such that the repl can propagate it's error here to be handled at the top
            panic!("{err:#?}")
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode().unwrap();
        loop {
            // creates an infinite loop unless you break it.
            if let Key(event) = read()? {
                println!("{event:?} \r");
                if let Char(c) = event.code {
                    //if let, is a pattern matching way in rust, event.code is actually like this Char('any character you typed'), so c gets assigned that character if its Char type
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode().unwrap();
        Ok(())
    }
}

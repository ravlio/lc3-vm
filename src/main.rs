use std::{env, process, thread, error};
use vm::VM;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use byteorder::{ReadBytesExt, LittleEndian};

fn main() -> Result<(), Box<dyn error::Error>> {
    if env::args().len() < 2 {
        println!("lc3 [image-file1] ...");
        process::exit(2);
    }

    env::args().next();

    let termios = Termios::from_fd(0).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &mut new_termios).unwrap();
    let vm = &mut VM::new();
    vm.load_image(env::args().collect::<Vec<String>>()[1].as_str())?;
    vm.run();

    tcsetattr(0, TCSANOW, &termios).unwrap();
    Ok(())
}
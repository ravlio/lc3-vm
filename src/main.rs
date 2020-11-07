use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use clap::{App, Arg};

mod vm;
mod memory;
mod error;
mod opcode;

fn run_vm(file: &str) -> Result<(), error::Error> {
    let termios = Termios::from_fd(0).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &mut new_termios).unwrap();
    let vm = &mut vm::new();
    vm.load_image(file).expect("can't load image");
    vm.run().expect("can't run vm");

    tcsetattr(0, TCSANOW, &termios).unwrap();
    Ok(())
}

fn main() -> Result<(), error::Error> {
    let matches = App::new("Little Computer 3")
        .about("LC3 compiler and virtual machine made for educational purposes")
        .version("0.1")
        .author("Maxim Bogdanov <muravlion@gmail.com")
        .subcommand(
            App::new("run")
                .about("load and run object file in VM")
                .arg(Arg::new("file").about("path to object file").required(true))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let file = run_matches.value_of("file").unwrap();
            run_vm(file)?;
        }
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    };

    Ok(())
}
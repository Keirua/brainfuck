use bf::run_program;
use bf::{InMemoryIO, StdIO};

fn main() {
    // prints "Z" with a loop
    let sample = "+++++++++[>++++++++++<-]>.";
    // let mut io = InMemoryIO::default();
    let mut io = StdIO {};
    run_program(sample, &mut io);
    // println!("{:?}", io);
}

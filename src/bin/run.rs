use bf::run_program;
use bf::InMemoryIO;

fn main() {
    // prints "Z" by incrementing 90 times
    // let sample = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.";
    // prints "Z" with a loop
    let sample = "+++++++++[>++++++++++<-]>.";
    let mut io = InMemoryIO::default();
    run_program(&sample, &mut io);
    println!("{:?}", io);
}

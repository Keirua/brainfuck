use bf::run_program;
use bf::StdIO;

fn main() {
    // let sample = "+++++++++[>++++++++++<-]>.";
    // let sample = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";
    // let mut io = InMemoryIO::default();
    let sample = include_str!("../../samples/0-to-99.bf");
    let mut io = StdIO {};
    run_program(sample, &mut io);
    // println!("{:?}", io);
}

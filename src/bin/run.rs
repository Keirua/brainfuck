use bf::bf::BrainfuckInterpreter;
use bf::io::StdIO;

fn main() {
    // let sample = "+++++++++[>++++++++++<-]>.";
    // let sample = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";
    // let mut io = InMemoryIO::default();
    let sample = include_str!("../../samples/0-to-99.bf");
    let mut io = StdIO {};
    let mut bf = BrainfuckInterpreter::new(sample);
    bf.run(&mut io);
    // println!("{:?}", io);
}

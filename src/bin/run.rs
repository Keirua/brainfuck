use bf::bf::BrainfuckInterpreter;
use bf::io::StdIO;

fn main() {
    // let sample = "+++++++++[>++++++++++<-]>.";
    // let sample = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";
    let sample = include_str!("../../samples/sierpinski.bf");
    // let sample = include_str!("../../samples/brainfuck.bf");
    let mut io = StdIO {};
    let mut bf = BrainfuckInterpreter::new(sample);
    bf.run(&mut io);
    // println!("{:?}", io);
}

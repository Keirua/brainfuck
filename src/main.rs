// Only 8 valid tokens
// +-<>[].,
// + Increment the value of the cell by 1
// - Decrement the value of the cell by 1
// > Move the pointer to the next cell to the right
// < Move the pointer to the next cell to the left
// . Output the ASCII character corresponding to the value of the current cell
// , Input a character and store its ASCII value in the current cell
// [ If the value of the cell is zero, jump to the corresponding ] character
// ] if the value of the current cell is non-zero, jump back to the corresponding [


fn main() {
    let mut memory = [0u8;256];
    let mut cell_id:usize = 0;
    let mut ip = 0usize;
    let sample = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++."; // prints "Z"
    while true {
        if ip < sample.len() {
            match sample.chars().nth(ip) {
                Some('+') => { memory[cell_id] += 1; },
                Some('.') => {
                    let v = memory[cell_id] as char;
                    println!("{v}"); },
                _ => panic!("Should not happen")
            }
            ip += 1;
        }
        else {
            break
        }
    }
    println!("{sample}");
}

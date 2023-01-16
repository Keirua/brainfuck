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

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod bf;
pub mod io;

extern crate byteorder;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod assembler;
pub mod instruction;
pub mod parser;
pub mod vm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

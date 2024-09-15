extern crate clip_core;
mod clipv {
   pub use clip_derive::*;
   pub use clip_core::*;
}
use clipv::parser::{Parsed, TryParse};
use clipv::TryParse;

#[derive(Debug, PartialEq, TryParse)]
enum Tata { One, Two, Three }
#[derive(TryParse)]
struct Toto {
    #[try_parse] tata: Tata,
    titi: u8
}


#[test]
fn doc_example_try_parse() {
     let arguments = [ "one", "32" ];
     let result = Toto::try_parse(arguments.iter());
     assert!(result.is_ok());
     let Parsed (parsed, _) = result.unwrap();
     assert_eq!(parsed.tata, Tata::One);
     assert_eq!(parsed.titi, 32);
}

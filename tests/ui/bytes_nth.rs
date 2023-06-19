//@run-rustfix

#![allow(clippy::unnecessary_operation)]
#![allow(clippy::slice_as_bytes)]
#![warn(clippy::bytes_nth)]

fn main() {
    let s = String::from("String");
    let _ = s.bytes().nth(3);
    let _ = &s.bytes().nth(3).unwrap();
    let _ = s[..].bytes().nth(3);
}

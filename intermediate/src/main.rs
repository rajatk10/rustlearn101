mod ref_borrow;
mod structs;
mod enums;
mod collections;
mod error_handling;
mod generics_traits;

fn main() {
    println!("$$$$$$$$$$  Welcome to Intermediate Rust Learning  $$$$$$$$$$$");
    println!("^^^^^  Here we will be using module to group respective topics  ^^^^^");
    println!("***************  Reference Vs Borrowing **********************");
    println!("As stated above this topic is a separate module ref_borrow");
    ref_borrow::demo();
    println!("****************  Structs & Impl ****************************");
    structs::demo();
    println!("****************  Enums  ****************************");
    enums::demo();
    println!("****************  Collections  **********************");
    collections::demo();
    println!("****************  Error Handling  *******************");
    error_handling::demo();
    println!("*****************  Generics & Traits  ***************");
    generics_traits::demo();
}

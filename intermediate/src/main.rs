mod ref_borrow;
mod learn_structs;
mod learn_enum;
mod collections;

fn main() {
    println!("$$$$$$$$$$  Welcome to Intermediate Rust Learning  $$$$$$$$$$$");
    println!("^^^^^  Here we will be using module to group respective topics  ^^^^^");
    println!("***************  Reference Vs Borrowing **********************");
    println!("As stated above this topic is a separate module ref_borrow");
    ref_borrow::demo();
    println!("****************  Structs & Impl ****************************");
    learn_structs::demo();
    println!("****************  Enums  ****************************");
    learn_enum::demo();
    println!("****************  Collections  **********************");
    collections::demo();
}

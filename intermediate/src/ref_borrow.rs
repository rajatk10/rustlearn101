//The function called from different crate has to be public as functions by default are private not visible outside module
pub fn demo() {
    println!("======= Loading the module =========");
    let s1 = String::from("Borrow");
    let len = calculate_length(&s1);
    println!("Length of Immutable string s1 = '{}' , is = {}", s1,len); //Can still access s1 as it is not moved but borrowed

    //Here is mutable
    let mut s2 = String::from("Reference To");
    println!("Length of mutable string s2 = '{}' , is = {}", s2,len); //S2 before it is mutated by reference"
    append_str(&mut s2);
    let len = calculate_length(&s2);
    println!("Length of mutable string s2 = '{}' , is = {}", s2,len); //S2 after it is mutated by reference - becomes "Reference To World"
    let slice: String = String::from("SLICES-LEARN");
    slices_str(&slice);
}

fn calculate_length(s: &String) -> usize {
    s.len() //return on s and this is a reference to s1
    //There is no ; so it is returned by default in rust
    //return s.len(); both are equivalent
}

fn append_str(s: &mut String) {
    s.push_str(" World");
}

fn slices_str(s: &str) {
    println!("Get slices of this string = {}", s);
    let s1 = &s[0..3];
    let s2 = &s[4..];
    println!("s1 = {} , s2 = {} i.e. slices from original string = {}", s1, s2, s);
    // s1 = &s[0..2]; Error as it is not mutable
}
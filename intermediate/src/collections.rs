
//This module will cover details for collections i.e vectors, hashmap, strings
use std::collections::HashMap;
pub fn demo() {
    println!("========  Learning Collections  =============");
    println!("--------  Vectors  --------------------------");
    vectors_demo();
    println!("-----  HashMap  ---------------");
    hash_map_demo();
    println!("-----  Strings  ----------------");
    strings_demo();
}

fn vectors_demo() {
    println!("Vectors are resizable arrays");
    //Unlike arrays we can append elements to it, push, pop, insert, remove
    let mut v1 = vec![1,2,3];
    println!("At beginning example has vector v1 = {:?}", v1); //mutable vector of type integer
    v1.push(4);
    println!("After pushing element vector becomes v1 = {:?}", v1);
    v1.pop();
    //push is LIFO add element at last and pop is also LIFO remove element from last
    println!("After popping element vector becomes v1 = {:?}", v1);
    v1.insert(1, 5);
    println!("Using insert in vector to add at any index example at 1 i.e v1 = {:?}", v1);
    v1.remove(2);
    println!("Using remove to delete element at 2nd , it becomes now v1 = {:?}", v1);
}

fn hash_map_demo() {
    println!("HashMap is a collection of key-value pairs");
    let mut h1 = HashMap::new(); //Creating new hashmap - import collections
    h1.insert("name", "Rust");
    h1.insert("type", "Programming Language");
    h1.insert("compiled", "true");
    h1.insert("popular", "true");
    println!("HashMap h1 = {:#?}", h1);
    //Accessing values from HashMap
    let name = h1.get("name");
    println!("Name = {} from hashmap using get function", name.unwrap());
    //Removing values from HashMap
    h1.remove("popular");
    println!("HashMap h1 = {:#?}, remove key 'popular'", h1);
    //handle panic error from unwrap if key is not found
    match h1.get("popular") {
        Some(value) => println!("Popular = {}", value),
        None => println!("Popular is not found in hashmap h1"),
    }
    //only insert if key doesn't exist
    h1.entry("popular").or_insert("true");
    println!("HashMap h1 after inserting unavailable key popular = {:#?}", h1);
    //Iterating over HashMap
    // for (key, value) in h1 {
    //     println!("{}: {}", key, value);
    // }
    //iterates by value (it consumes the HashMap). After that loop finishes, h1 is no longer usable,
}

fn strings_demo() {
    println!("Strings are resizable arrays");
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("After cloning s1 = {} , s2 = {}", s1, s2);
    //As it is clone both s1 and s2 are accessible
    //But if we do s2 = s1 then s1 will be moved to s2 and s1 will not be accessible
    let s2 = s1;
    println!("After copying reference s2 = {}", s2);
    // println!("s1 = {}", s1); //This will cause error as s1 is moved to s2
    //To avoid moving we can use reference
    let s3 = &s2;
    println!("s3 = {}", s3); //all point to same memory - variable
    let string2 = String::from("Rust");
    for c in string2.chars() {
        println!("Iterating over each char of string2 = {} break after first", c);
        break
    }
    //string2.bytes help to print bytes iteratively
}
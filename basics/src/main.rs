use std::collections::HashMap;
//use std::io::{stdin, Read};

fn main() {
    println!("$$$$$$$$$$$$$$$$$$  Basics of Rust - Language $$$$$$$$$$");

    println!("******************  01 - Variables  ******************");

    let x = 5; // immutable variable
    println!("{}", x);
    //error - ^^^^^ cannot assign twice to immutable variable
    //x = 6; //Cannot assign a new value to an immutable variable

    //Assignment
    let mut x = 50; //mutable variable
    println!("{}", x);
    x = 6;
    println!("{}", x);

    //strings 01
    let mut s1 = String::from("hello");
    s1.push_str(" world");
    println!("{}", s1);

    //string 02
    let mut s11:&str = "hello";
    //s11.push_str(" world"); - If need to modify use String instead of reference
    println!("{}", s11);
    s11 = "hello world from strings";
    println!("{}", s11);

    //Difference between String and &str
    // Use &str when:
    //you just need to read text , you want to avoid allocations , you’re passing string data into functions
    // Use String when:
    //you need to modify the text , you want to own the data , you’re building a string dynamically

    println!("******************  02-Data Types  *******************");
    println!("Primitive i.e integer, float, char, boolean etc");
    println!("Non-Primitive i.e vector, hash map, hash set, Box, Rc, Arc, ");
    println!("Compound types i.e. tuple, array, slice, str");

    //Primitive
    // integer - Signed/Unsigned , float - f32/f64 , boolean - true/false, character - char , references &T, &mut T, raw pointers *mut T, *const T
    // Compund types : tuple - mixed data types (fixed size) (i32,f64,&str) , array (fixed size) of same type , slice [&T] , str
    // Non-Primitive
    // vector, hash map, hash set, Box, Rc, Arc,
    let t1:(i32,f64,&str) = (1,2.0,"hello"); //packing tuple
    println!("Tuple Example : t1 = {}, {}, {}", t1.0, t1.1, t1.2);
    let (a,b,c) = t1; //Unpacking tuple
    println!("Unpacking tuple t1 i.e. a = {}, b = {}, c ={}", a, b, c);
    //Error if tuple is not mutable
    //t1.0 = 10; - Error - cannot assign to tuple element because it is not declared as mutable
    let mut t11:(f32,i32,String) = (1.0,2,String::from("hello"));
    println!("Mutable tuple t11 : {:#?}", t11); // this prints each on new line- handy
    t11.2 = String::from("world");
    println!("Mutate above tuple 3rd element string hello : t11 = {:?}", t11);

    let mut a1:[i32; 10] = [1,2,3,4,5,6,7,8,9,10]; //{:?} is a placeholder that will be replaced by a value you pass after the string.
    println!("Array Example : a1 = {:?}", a1);
    //Edit array now as it is mutable
    a1[0] = 10;
    println!("Mutate above array example : a1 = {:?}", a1);
    //Error if array is not mutable
    //let b1:[f32; 5] = [1.0,2.0,3.0,4.0,5.0];
    //b1[0] = 10; - Error - cannot assign to array element because it is not declared as mutable

    let mut v1  = vec![1,2,3,4,5]; //vec! is the macro
    println!("Vector Example : v1 = {:?}", v1);
    v1.push(6);
    println!("Vector Push 6 into : v1 = {:?}", v1);
    v1.push(7);
    println!("Vector Push 7 and pop  : v1 = {:?}", v1);
    v1.pop(); //Last in First out as this is in stack
    println!("Vector Post pop event : v1 = {:?}", v1);

    let mut h1:HashMap<&str,i32> = HashMap::new();
    h1.insert("a",2);
    h1.insert("b",4);
    h1.insert("c",6);
    println!("HashMap Example : h1 = {:#?}", h1);
    //Other hashmap methods include -
    println!("Hashmap h1 get value of key 'a' = {:?} ", h1.get("a").unwrap());
    //h1.contains_key("b"); //h1.remove("c"); h1.len(); h1.capacity(); h1.clear(); h1.iter();

    for (key, value) in h1.iter() {
        println!("Print/Access hashmap values h1 using loop {}: {}", key, value);
    }
    println!("******************  03 - If Else - Functions *******************");
    println!("Call function odd even, take input from user");
    odd_even();


    println!("******************  04 - Control Flow *****************************");
    println!("=======Loops are of 3 types here infinite `loop` , `for`, `while` ");
    let mut loopn1:i32 = 5;
    loop {
        println!("Infinite loop Example iteration {}" , loopn1);
        if loopn1 >= 7 {
            println!("Thala for a Reason, breaking infinite loop");
            break
        }
        loopn1 += 1;
    }

    let mut whilen1:i32 = 5;
    while whilen1 <= 7 {
        println!("While loop Example iteration {}" , whilen1);
        whilen1 += 1;
    }

    for i in 5..=7 { //1..3 - 3 not included, 1..=3 included
        println!("For loop Example iteration {}" , i);
    }

    println!("******************  05-Stack vs Heap  ************************");
    println!("Stack is used for fixed size data and Heap is used for dynamic size data, while stack is faster is only in few MBs , heaps can grow");
    let s1 = String::from("hello"); //s1 is stored in heap
    let s2 = s1; // s2 handler is on stack, the data it points to is on heap
    println!("String on heap memory is = {}", s2);
    //println!("{}", s1); Value is moved after being moved

    let s11: &str = "hello";
    let s22 = s11;
    println!("{}", s22);
    println!("{}", s11); //Here value is refrenced both are pointing to same memory location
    println!("Printing memory location s11 = {:p} , s22 = {:p} , only address as {:p} " , s11, s22, s11.as_ptr());
    // Use s11.as_ptr() and s22.as_ptr() for the bytes address
    // Use &s11 and &s22 for the variable address (where the fat pointer lives)
    println!("Variable address of s11 = {:p} , s22 = {:p} ", &s11, &s22);
    println!("Even though the values stored inside those variables are identical (same fat pointer), they are still two separate variables, so they occupy two different stack locations.");

    println!("=========== Example 2nd ==============");
    let mut s111 = String::from("Hello");
    println!("String handle (stack) address   =  {:p} ", &s111);
    println!("String heap address  = {:p} ", s111.as_ptr());
    println!("Length of string = {} , capacity of string = {}", s111.len(), s111.capacity());
    println!("Now mutating string ");
    s111.push_str(" World");
    s111.push_str(" From Stack and Heap ");
    s111.push('!');
    println!("Mutated string = {} after push", s111);
    println!("String handle (stack) address   =  {:p} ^^^this would be same as earlier ", &s111); //You will not this doesn't change
    println!("String heap address  = {:p} ", s111.as_ptr()); //this may change after several push operations
    println!("Length of string = {} , capacity of string = {}", s111.len(), s111.capacity());

    println!("=========== Example 3rd ==============");
    move_vs_clone();

}

fn odd_even() {
    //Read input from user, check if it is valid int and process the odd even check -
    // let mut input:String = String::new(); //Cannot borrow input if it is not declared as mut
    // stdin().read_line(&mut input);
    // let parsed = input.trim().parse::<i32>();
    // //Result is a enum and Ok & Err are two variant cases of this enum. Result<T, E> = Ok(T) | Err(E)
    // let num = match parsed {
    //     Ok(num) => num,
    //     Err(e) => {
    //         println!("Please enter a valid integer ==> Error: {:?} i.e. Retry again", e);
    //         return;
    //     }
    // };
    let num = 10;
    if num == 0 {
        println!("You have entered 0, Please try again")
    } else if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}

fn move_vs_clone() {
    println!(" Move vs Clone Operations w.r.t Srack and Heap ");
    let a = String::from("Move Example");
    println!("Address of a = '{}' is = {:p}", a, a.as_ptr());
    let b = a;
    //println!("A becomes invalid now as it is moved {}" , a)
    println!("Address of b = '{}' after it is moved = {:p} , will be same as a ", b, b.as_ptr());

    let c = String::from("Copy Example ");
    let d = c.clone();
    println!("Address of c i.e '{}' is {:p}", c,c.as_ptr());
    println!("Address of d = '{}' , is {:p} after cloning with c is different as it is cloned", d, d.as_ptr());

}
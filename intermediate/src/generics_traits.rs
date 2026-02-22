
//Here we will try to understand traits and generics
use std::fmt::Debug;
//Why this is needed?  because Debug is not in prelude - see bottom for more details on prelude
pub fn demo() {
    println!("========  Learning Generics =============");
    //generics_demo();
    let gen1 = max(100,102);
    println!("Using Ordered generics - Max value is {}", gen1);
    println!("========  Learning Traits =============");
    traits_demo();
    println!("--------  Traits Example 2  -----------");
    debug_ord_trait_demo(100,201);
}
trait Wheels {
    fn num_wheels(&self) -> u32;
}
struct Car;
struct Bike;

impl Wheels for Car {
    fn num_wheels(&self) -> u32 {
        4
    }
}
impl Wheels for Bike {
    fn num_wheels(&self) -> u32 {
        //todo!()
        2
    }
}

fn max<T: Ord>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn describe_vehicle<T: Wheels>(v: &T) {
    println!("---------  Generic Type  -----------------");
    println!("This vehicle has {} wheels ", v.num_wheels());
}



fn traits_demo() {
    println!("A trait is a rust's way to describe shared behaviour - ex - interfaces in java/go");
    println!("A trait defines a set of methods that a type must implement");
    println!("A trait can be implemented for any type");
    //enum & struct = data / shape
    //trait = behaviour

    //Now need instantiation and calling above
    let car = Car;
    let bike = Bike;
    // println!("Car has {} wheels", car.num_wheels());
    // println!("Bike has {} wheels", bike.num_wheels());

    describe_vehicle(&car);
    describe_vehicle(&bike);
    println!("Most common traits you will see are - ");
    println!("1. Clone, 2. Copy, 3. Debug, 4. Display, 5. PartialEq, 6. Eq, 7. PartialOrd, 8. Ord, 9. Hash ");
    //T: Clone, T: Copy, T: Debug, T: Display, T: PartialEq, T: Eq, T: PartialOrd, T: Ord, T: Hash
    //Derive is used at times to implement these traits automatically by compiler.
    //#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[derive(Clone, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    println!("Here is clone trait example i.e. p1 = {:?} where x = {}, y = {}, p2 = {:?}", p1,p1.x, p1.y, p2);
}
fn debug_ord_trait_demo<T: Debug + Ord>(x: T, y: T) {
    println!("Debug and Ord trait example - x = {:?}, y = {:?}", x, y);
    if x > y {
        println!("{:?} is greater than {:?}", x, y);
        //T doesnot implement Display so need to use it with :? as debug is already in scope
    } else {
        println!("{:?} is less than {:?}", x, y);
    }

}
// What's the "prelude"?
// Rust automatically imports certain commonly-used items into every module without you having to write use. This is called the prelude.
//
// The prelude includes:
//
// Option, Result
// Vec, String
// Ord, PartialOrd, Eq, PartialEq
// Copy, Clone
// Drop
// Many others
// But it does NOT include:
//
// Debug (in std::fmt)
// Display (in std::fmt)
// Iterator (in std::iter)
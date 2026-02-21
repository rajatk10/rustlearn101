

struct Employee {
    id : u32,
    name: String,
    age: u8,
    salary: u32,
    designation: String
}
pub fn demo() {
    println!("=========  Learning Structs Demo Examples  =============");
    let mut e1 : Employee = Employee {
        id : 1,
        name: String::from("John Doe"),
        age: 30,
        salary: 50000,
        designation: String::from("AMTS")
    };
    println!("Employee ID = {} , Name = {} , Age = {} , Salary = {} , Designation = {}", e1.id, e1.name, e1.age, e1.salary, e1.designation);
    e1.designation =  String::from( "MTS");
    println!("Promoted the employee to SDE2 , newer designation = {}", e1.designation);
    println!("Using the earlier struct Employee to create a new struct called employee");
    let e2 = Employee {
        name: String::from("Jane Doe"),
        id: 2, // here order does not matter
        ..e1 //Using updated e1 as base
    };
    println!("Employee ID = {} , Name = {} , Age = {} , Salary = {} , Designation = {}", e2.id, e2.name, e2.age, e2.salary, e2.designation);
    println!("========  Example of implementation in rust  ==============");
    let rect1 = Rectangle {
        width: 10,
        height: 5
    };
    rect1.display();
    rect1.area();
    println!("========  Example of associated function in structs ==========");
    let sq = Rectangle::square(30);
    println!("Area of square using associated function");
    sq.area();
}

//  In rust methods are those functions which are applied to a type example on structs or enums, or any other defined type
//  How it differs
// Normal function: standalone, called like do_something(x)
// Method: belongs to a type, called like x.do_something(), it will have &self as first parameter - just like python
//&self (borrow the object, read-only)
//&mut self (borrow mutably, can modify the object)
//self (take ownership of the object)


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn display(&self) {
        println!("Rectangle is a struct with width = {} , height = {}", self.width, self.height);
    }
    fn area(&self) {
        println!("Area of rectangle = {}", self.width * self.height);
    }

    //Associated functions in struct
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

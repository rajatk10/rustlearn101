
//Struct is to group multiple fields together which exists together at all time example employee in previous module
//enum is to group multiple variants together which exists at different times
//Add enum and implementation at module scope so each function can access
//make it pub - if you want others module to access too
#[derive(Debug)] //This to tell Rust compiler to implement Debug trait for this enum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
//both enum (data, shape ) and impl (behaviour) refer to same type which is IpAddr
impl IpAddr {
    fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(a,b,c,d) => (*a,*b,*c,*d)  == (127,0,0,1),
            IpAddr::V6(s) => s.as_str() == "::1",
        }
    }
}

pub fn demo() {
    println!("========  Learning Enum Demo Examples  =============");
    // here :: is used to access things on a type / namespace (Associated items)
    // . is used to access things on a value/instance (methods/fields)

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Enum for IpAddr , V4 is set to {:?} and V6 is set to {:?}", home , loopback);
    //Without drive[Debug] the above print would fail as IpAddr doesn't have display, so we will add it currently.
    //Other way to access and print these values is via match
    match &home {
        IpAddr::V4(a, b, c, d) => println!("Print IPV4 with match expression = {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(s) => println!("{}", s), //This will be ignored - better to have for rust safety feature - it also causes exhaustive matching
    }
    //Other way to access and print these values is via if let
    match &loopback {
        IpAddr::V4(a,b,c,d)  => println!("Ip Address V4 is = {}.{}.{}.{}", a , b, c, d), //this will be ignored - here for rust safety
        IpAddr::V6(s) => println!("Match Expression - IP V6 Address is = {}", s),
    }
    //Other way to access and print these values is via if let
    if let IpAddr::V4(a,b,c,d) = &home {
        println!("One more way to access enum i.e without match expression ");
        println!("Ip Address V4 is = {}.{}.{}.{}", a , b, c, d);
    }

    //Now when it comes to access or mutate the enums - there are 3 main things 1. take ownership - generate new value, mutate in place, or just use it

    println!("loopback V4 matches 127.0.0.1 i.e. = {}", home.is_loopback());
    println!("*************  2nd Enum Example  *********************");
    coin_enum();
    println!("*************  3rd Enum Example  *********************");
    println!("Type - Complex Enum of mixed data types");
    enums_complex_demo();
}

enum Coin {
    Paisa(String),
    Rupee(u8),
    FiveRupee(u8),
}

impl Coin {
    fn value_is(&self) {
        match self {
            Coin::Paisa(coin) => println!("Coin {} is Paisa", coin),
            Coin::Rupee(coin) => println!("Coin {} is Rupee", coin),
            Coin::FiveRupee(coin) => println!("Coin {} is FiveRupee", coin),
       }
    }
}

fn coin_enum() {
    let coin1 = Coin::FiveRupee(5);
    let coin2 = Coin::Paisa(String::from("50p"));
    let coin3 = Coin::Rupee(10);
    coin1.value_is();
    coin2.value_is();
    coin3.value_is();
}

struct Employee {
    id : u32,
    name: String,
    role: Role,
}

enum Role {
    IndividualContributor {level: u8},
    Manager,
    //Executive(String, i32),
}

impl Employee {
    fn check_role(&self) {
        match &self.role {
            Role::IndividualContributor {level} => println!("Employee is a IC, at level {}", level),
            Role::Manager => println!("Employee is manager"),
            //Role::Executive(a,b) => println!("Executive belongs to team = {} strength = {}", a, b),
        }
        println!("His/Her name is {} and id = {}", &self.name, &self.id);
    }
}

fn enums_complex_demo() {
    let e1_role1 = Role::IndividualContributor {level: 7};
    let e1:Employee = Employee {
        id: 1,
        name: String::from("Amitesh"),
        role: e1_role1,
    };
    e1.check_role();
    let e2_role2 = Role::Manager;
    let e2 = Employee {
        id: 2,
        name: String::from("Shashank"),
        role: e2_role2
    };
    e2.check_role()


}
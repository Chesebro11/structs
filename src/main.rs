// Structs are pretty similar to sql, give this thing some definitions and then fill out somewhere else
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
//     user1.email = String::from("anotheremail@example.com");
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
//---------------------------------------------------------------------
// Another use for structs, defining the values of color and point then later filling those values.
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// You can define structs that dont have any fields, these are called unit=like structs
// struct AlwaysEqual;

// fn main () {
//     let subject = AlwaysEqual;
// }
//---------------------------------------------------------------------
// Simple area calculation program, needs refactoring
// fn main () {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
//---------------------------------------------------------------------
// This is refactored with tuples, refactoring further with sctructs.
// fn main () {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
//---------------------------------------------------------------------
// This is the same program but refactored for structs. While initially it looks
// like more coding, on a larger scale program this can reduce code significantly
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main () {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
//---------------------------------------------------------------------
// this allows us to view the actual values of Rectangle, cant do without derive debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// everything within this code block is called an associated function.
// using &self so that we arent taking ownership
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // adding another method to this block, this comapres the dimensions of 2 rectangles
    // to determine if one can contain the other 
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >  other.width && self.height > other.height
    }
}

fn main () {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // changing {:?} can alter the format of this output
    // println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
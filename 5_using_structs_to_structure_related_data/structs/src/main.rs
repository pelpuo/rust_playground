#[derive(Debug)] //Attribute to enable printing
struct Rectangle {
    width: u32,
    height: u32
}

// Method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {

    // Using separate width and height
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    
    // Using tuple
    let rect_tuple = (30,50);
    println!(
        "The area of the tuple rectangle is {} square pixels.",
        area_tuple(rect_tuple)
    );


    // Using Struct
    let scale = 3;
    let rect_struct = Rectangle {
        width: dbg!(10 * scale), // Add dbg! to print to stderr
        height: 50,
    };
    
    let rect_struct2 = Rectangle {
        width: 2,
        height: 40,
    };

    println!(
        "The area of the struct rectangle is {} square pixels.",
        area_struct(&rect_struct)
    );

    println!("rect1 is {rect_struct:?}"); //Print out debugging info

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_struct.area()
    );

    println!("Can rect1 hold rect2? {}", rect_struct.can_hold(&rect_struct2));

    let rect_struct3 = Rectangle::square(12);
    println!("Square from rect_struct3 {}", rect_struct3.area());

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
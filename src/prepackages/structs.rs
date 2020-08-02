
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn structure() {

    // first_structs();

    rectangles();
}

fn first_structs() {
    // Create a mutable user.
    let mut first_user = User {
        email: String::from("harry.houdini@tamere.fr"),
        active: true,
        sign_in_count: 1,
        username: "Harry1874".to_string(),
    };
    // See? Mutable.
    first_user.sign_in_count += 1;

    // 'Struct update syntax' example.
    let second_user = User {
        email: "new.email@address.org".to_string(),
        username: "HenkieIsNieGek".to_string(),
        ..first_user    // ...Rust's version of spread operator
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

fn build_user(email: String, username1: String) -> User {
    User {
        email,  // Shorthand notation, equal to below field assignment. ONLY WORKS if parameter equals field name.
        username: username1,    // Necessary because parameter name and field do not match.
        sign_in_count: 1,
        active: true,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Circle {
    diameter: u32
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn longest_side(&self) -> u32 {
        if self.width > self.height {
            return self.width
        }
        self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Transforms this Rectangle into a circle, invalidating the originating Rectangle.
    fn transform_to_circle(self) -> Circle {
        Circle { diameter: (self.height + self.width) / 2 }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn stretch(&mut self, add_height: u32, add_width: u32) {
        self.height += add_height;
        self.width += add_width
    }

    // Associated function (sorta static)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

// Totally fine to randomly define another method in a separate impl block.
impl Rectangle {
    fn shrink(&mut self, take_height: u32, take_width: u32) {
        self.height -= take_height;
        self.width -= take_width
    }
}

fn rectangles() {
    let tiananmen_square = Rectangle { width: 500, height: 880 };

    println!("{:?}", tiananmen_square);
    println!("Bit fancier/more exploded: {:#?}", tiananmen_square);

    println!("Area of rectangle, calculated by calling function area() is: {}", area(&tiananmen_square));
    println!("Area of rectangle, calculated by calling method Rectangle.area() is: {}", area(&tiananmen_square));

    println!("Longest side of rectangle is: {}", tiananmen_square.longest_side());

    let mut trafalgar_square = Rectangle {
        ..tiananmen_square
    };
    trafalgar_square.shrink(200, 300);

    println!("Trafalgar square area: {}", trafalgar_square.area());

    let actual_square = Rectangle::square(144);
    println!("Actual squares have equal height and width: {:#?}", actual_square);
    println!("Actual square area: {}", actual_square.area());

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

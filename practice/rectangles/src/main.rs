#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        println!("\nself has height {} and width {}.", self.height, self.width);
        println!("other has height {} and width {}.\n", other.height, other.width);
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}

fn main() {

    let r1 = Rectangle {
        height: 50,
        width: 30,
    };
    let r2 = Rectangle {
        height: 40,
        width: 10,
    };
    let r3 = Rectangle {
        height: 30,
        width: 15,
    };

    println!(
        "The area of rectangle 1 with width of {} and height of {} is {}.",
        r1.width,
        r1.height,
        r1.area()
    );
    println!(
        "The area of rectangle 2 with width of {} and height of {} is {}.",
        r2.width,
        r2.height,
        r2.area(),
    );
    println!(
        "The area of rectangle 3 with width of {} and height of {} is {}.",
        r3.width,
        r3.height,
        r3.area(),
    );

    println!("Can rectangle 1 hold rectangle 2? {}\n", r1.can_hold(&r2));
    println!("Can rectangle 2 hold rectangle 3? {}\n", r2.can_hold(&r3));

    print!("Making a square with side size of 5: ");
    let sq = Rectangle::square(5);
    println!("side 1 = {}, side 2 = {}", sq.width, sq.height);
}
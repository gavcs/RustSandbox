#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let scale = 2;

    let r1 = Rectangle {
        length: dbg!(25 * scale),
        width: 40,
    };
    let a = area(&r1);
    println!("r1 = {:#?}\n", r1);
    println!("The area is {a}\n");
    dbg!(&r1);
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.length
}
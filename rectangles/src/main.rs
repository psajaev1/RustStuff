#[derive(Debug)]


struct Rectangle {
    width: u32,
    height: u32,
}




fn main() {
    
    let r1 = Rectangle {
        width: 9,
        height: 11,
    };

    println!("Area is {}", get_area(&r1));
    println!("{:?}", r1);
}

fn get_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

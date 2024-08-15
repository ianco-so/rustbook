#[derive(Debug)]
struct Retangle {
    width: u32,
    height: u32,
}

impl Retangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        (self.width >= other.width || self.height >= other.height) && self.area() >= other.area()
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn new(width: u32, height: u32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Width and Height must be greater than 0");
        }
        Self {width, height}
    }

    // fn set_width(&mut self, width: u32) {
    //     self.width = width;
    // }

    // fn set_height(&mut self, height: u32) {
    //     self.height = height;
    // }
}

fn main() {
    let rect1 = Retangle::new(0, 50);
    
    let rect2 = Retangle::new(10, 20);
    let rect3 = Retangle::new(60,45);
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rect1 ({} X {}) is {}².",
        rect1.get_height(),
        rect1.get_width(),
        rect1.area()
    );

    println!(
        "The area of the rect2 ({} X {}) is {}².",
        rect2.get_height(),
        rect2.get_width(),
        rect2.area()
    );

    println!(
        "The area of the rect3 ({} X {}) is {}².",
        rect3.get_height(),
        rect3.get_width(),
        rect3.area()
    );

    println!(
        "Can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "Can rect1 hold rect3? {}",
        rect1.can_hold(&rect3)
    );
}

use text_io::read; //to use macro read! to get easy user input

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Input rectangle width: ");
    rect.width = read!();
    println!("Input rectangle height: ");
    rect.height = read!();

    if rect.width == rect.height {
        println!("The rectangle is a square!")
    }
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}
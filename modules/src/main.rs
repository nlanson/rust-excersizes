mod Boxes {
    pub struct OpenBox<T> {
        pub contents: T
    }

    pub struct ClosedBox<T> {
        contents: T
    }

    impl<T> OpenBox<T> {
        //Public constructor method for OpenBox
        pub fn new(contents: T) -> OpenBox<T> {
            OpenBox {
                contents: contents
            }
        }
    }
    
    impl<T> ClosedBox<T> {
        //Public constructor method for ClosedBox.
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents
            }
        }
    }
}


fn main() {
    //Create new variable using the OpenBox constructor in the Boxes module.
    let open_box = Boxes::OpenBox::new("shoe");
    println!("Open box contains a {}", open_box.contents);

    //Open box has no private fields so it can be created without a constructor method as well.
    let second_open_box = Boxes::OpenBox{contents: "ham"};
    println!("Second open box contains some {}", second_open_box.contents);

    //Closed box needs to be constructed with a constructor method becuase you cant set private fields.
    let closed_box = Boxes::ClosedBox::new("secret sauce");
    println!("We cant see the contents of the closed box because it is private :(");
}

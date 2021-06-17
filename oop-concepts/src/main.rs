mod Life {
    
    pub struct Animal<N, A> {
        name: N,
        age: A
    }

    pub struct Dog {
        breed: String,
        micro_chip: i64
    }

    impl Dog {
        pub fn new(breed: String, micro_chip: i64) -> Dog {
            Dog {
                breed: breed,
                micro_chip: micro_chip
            }
        }

        pub fn bark(&self, words: &str) {
            println!("bark bark.... {} .... bark bark im a dog", words)
        }
    }   
}

fn main() {
    let myDoggy = Life::Dog::new(String::from("Choco"), 34);
    myDoggy.bark("im not a dog lol");
}

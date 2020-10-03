use std::io; //import standard input/output method

fn main() {
    println!("-------------------");
    println!("Fibonacci Printer!");
    println!("-------------------");

   let count: u32 = get_count();

   println!("----------------------------------------------");
   println!("Fibonacci printer will go brrrrrrr {} times!", count);
   println!("----------------------------------------------");

   print_fibo(count);

   println!("--------------------------------");
   println!("Fibonacci printer has finished!");
   println!("--------------------------------");

   let mut exit_code = String::new();
   println!("Press Return to exit.");
   io::stdin()
            .read_line(&mut exit_code)
            .expect("Cant Exit :(");
}//end main

fn get_count() -> u32 { //function will return a u32 value
    let final_count: u32 = loop { //assign variable final_count as a u32 type. Make the value of this variable to result of the loop.
        let mut count = String::new(); //new mutable variable named count = new String.
        println!("How many Fibonacci's do you want to print?");
        io::stdin() //input/output method (standard input)
            .read_line(&mut count) //read line and allocate it to mutable string variable count from line 17
            .expect("Invalid amount of Fibonacci's :("); //error handling
        
        let count: u32 = match count.trim().parse() { //convert the string variable count to a u32 interger by trimming and parsing count and then matching if the trim and parse returns Ok or Err.
            Ok(num) => num, //if ok, assign count = num
            Err(_) => { //if err, do the loop again
                continue 
            }, //end Err
        };//end count match
        break count; //return variable count from loop to final_count
    };//end loop
    final_count //return final_count to main fn count
}//end fn get_count

fn print_fibo(count: u32) {
    let mut i: u32 = 1;
    
    let mut x: u64 = 0; //fixed to u32 from i64 to allow for more int printing.
    let mut y: u64 = 1;
    let mut z: u64 = 0;
    
    while i <= count {
        println!("{}", x);
        z = x + y;
        x = y;
        y = z;
        i += 1;
    }
}

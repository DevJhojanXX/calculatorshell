fn main() {
    println!("Hello, world!");
    println!("Result {}", result(2,4,"+")); 
}

fn result(number_one:i32, number_two:i32, operation:&str)->i32{
    match operation {
       "+" => number_one + number_two, 
       "-" => number_one - number_two, 
       "x" => number_one * number_two, 
       "/" => number_one / number_two, 
       "%" => number_one % number_two,
       _=> {
            println!("Invalid operation");
            return -00000;
       }
    } 
}
struct BasicOperations {
    result:f64,
    numbers_one: Vec<String>,
    numbers_two: Vec<String>,
    operators: Vec<char>,
    num: String,
}

impl BasicOperations {
    fn new()-> Self{
        BasicOperations{
            result: 0.0,
            numbers_one: Vec::new(),
            numbers_two: Vec::new(),
            operators: Vec::new(),
            num: String::new(),
        }
    }
    
    fn data_management(&mut self, operation:&str) -> f64{
        let operation_chars: Vec<char> = operation.chars().collect();
        for &data in &operation_chars{
            if self.numbers_two.is_empty() && self.operators.is_empty(){
                if data.is_digit(10) || data == '.'{
                    self.num = data.to_string();
                    self.numbers_one.push(self.num.clone());
                }
            }
            if data == '+' || data == '-' || data == 'x' || data == '/' || data == '%'{
                self.calculated_operation();
                self.operators.push(data);
            }
            else if !self.operators.is_empty() && !self.numbers_one.is_empty(){
                if data.is_digit(10) || data == '.'{
                    self.num = data.to_string();
                    self.numbers_two.push(self.num.clone());
                }
            }
        }
        self.calculated_operation(); 
        self.result
    }
    
    fn calculated_operation(&mut self){
        if !self.numbers_one.is_empty() && !self.numbers_two.is_empty(){
            self.result = self.calculated(&self.numbers_one, &self.operators, &self.numbers_two);
            self.numbers_one.clear();
            self.num = self.result.to_string();
            self.numbers_one.push(self.num.clone());
            self.numbers_two.clear();
            self.operators.clear();
        }
    }
    
    fn calculated(&self, numbers_one:&[String], operators:&[char], numbers_two:&[String])->f64{
        let number_one_string: String = numbers_one.join("");
        let number_two_string: String = numbers_two.join("");
        let number_one: f64 = number_one_string.parse().unwrap();
        let number_two: f64 = number_two_string.parse().unwrap();
        let operator: char = operators[0];
        match operator {
            '+' => return number_one + number_two, 
            '-' => return number_one - number_two, 
            'x' => return number_one * number_two, 
            '/' => return number_one / number_two, 
            '%' => return number_one % number_two,
            _=> {
                println!("Invalid operation");
                return 0.0;
            }
       }
    } 
}
fn main(){
    let mut basic_ops:BasicOperations = BasicOperations::new();
    let operation:&str = "(5+2+3)/3"; 
    let result:f64 = basic_ops.data_management(operation); 
    println!("Result: {}", result);
}

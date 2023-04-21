
use std::{io};

fn promptoperand() -> io::Result<String>{
    println!("Operand ");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)?;
      
    Ok(user_input)
}
fn promptoprator() -> io::Result<String>{
    println!("Operator");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)?;
    let operator = &user_input[0..1];
    println!("{:?}",operator);
    if !(operator == "/" || operator == "+" || operator == "-" ||  operator == "*"){
        return Ok(String::new());
    }
    Ok(operator.to_owned())
}

fn fraction(input : &str) -> f64{
    let fixed = input.split("|").map(|f| f.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    if(fixed.len() != 2){
        panic!("Invalid float");
    }
    fixed[0] as f64 / fixed[1] as f64
}

fn real(input : &str) -> f64{
    let mut fixed = input.replace(",", ".").parse::<f64>().unwrap();
    
    fixed
}

fn check(first : &str) -> f64{
    let mut number: Option<f64> = None;
    if first.contains("|"){
        number = Some(fraction(&first));
    }else if (first.contains(",")) {
        number = Some(real(&first));
    }else{
        let mut formatted = String::from(first);
        formatted.push_str(".0");
        number = Some(formatted.parse::<f64>().unwrap());
        println!("{:?}",number)
    }
    number.unwrap()
}


fn calc() -> Result<f64,String> {
    let mut first = promptoperand().unwrap();
    first = String::from(&first[0..first.len()-2]);
    
    let number1 = check(&first);
    
    let operator = promptoprator().unwrap();
    if operator.is_empty(){
        return Err("Invalid operator".to_owned());
    }
    let mut second = promptoperand().unwrap();

    second = String::from(&second[0..second.len()-2]);
    let number2 = check(&second);
    //check
    let mut result: Option<f64> = None;
    if(operator == "+"){
        result = Some(number1 + number2);
    }
    if(operator == "-"){
        result = Some(number1 - number2);
    }
    if(operator == "*"){
        result = Some(number1 * number2);
    }
    if(operator == "/"){
        result = Some(number1 - number2);
    }
    

   Ok(result.unwrap())
}
fn main() {
    loop {
        match calc(){
            Ok(x) => println!("Result {x}"),
            Err(x) => println!("Invalid : {x}")
        }
    }
}

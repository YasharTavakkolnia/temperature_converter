use std::io;

fn main(){

println!("please Enter option: \n 1 : F -> C  \n 2 : C -> F");

let mut option : String = String::new();

io::stdin()
    .read_line(&mut option)
    .ok()
    .expect("Couldn't read line");

let option : i32  =option.trim().parse().expect("invalid input");

if option == 1{

let mut fahrenheit : String = String::new();

println!("please enter fahrenheit: ");
io::stdin()
    .read_line(&mut fahrenheit)
    .ok()
    .expect("Couldn't read line");
let fahrenheit : i32  =fahrenheit.trim().parse().expect("invalid input");
    println!("this is F To C: {}",fahrenheit_to_celsius(fahrenheit).to_string());

    
}else if option == 2{
    let mut celsius : String = String::new();
    println!("please enter celsius: ");
    io::stdin()
        .read_line(&mut celsius)
        .ok()
        .expect("Couldn't read line");
    let celsius : i32  =celsius.trim().parse().expect("invalid input");
        println!("this is C To F: {}",celsius_to_fahrenheit(celsius).to_string());

    }else{
    println!("invalid input");
}
}



fn fahrenheit_to_celsius(fahrenheit:i32)-> i32{

        let celsius : i32 = ((fahrenheit -32)*5) / 9; 
        return celsius;
        
}
fn celsius_to_fahrenheit(celsius: i32)-> i32{

    let fahrenheit : i32 = ((celsius * 9)/5) + 32; 
    return fahrenheit;
    
}
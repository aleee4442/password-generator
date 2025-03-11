// libraries used
use rand::Rng; 
use std::io;

fn generate_password(lenght: u32){
    let mut rng = rand::rng(); // for the random number generator

    // all the possible characters in the password
    let digits: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect(); 

    // generating the password
    let mut password = String::new();
    for _ in 0..lenght{
        let indice = rng.random_range(0..digits.len()); // using digits len you can add more digits with no more changes
        password.push(digits[indice]);
    }
    println!("{}",password); // shows the generated password
}


fn main() {
    // variable for selecting the number of passwords to generate
    let mut password_number = String::new(); 
    
    println!("Welcome to the safe password generator");
    
    println!("Input the number of passwords to generate");
    io::stdin().read_line(&mut password_number).expect("Error reading the line");
    let opcion_u32 = password_number.trim().parse::<u32>().expect("Error, number not introduced");
    
    for i in 0..opcion_u32{
        let mut lenght_inserted = String::new();
        
        println!("GGenerating password {}", i+1);
        println!("Choose the lenght of the password");
        io::stdin().read_line(&mut lenght_inserted).expect("Error reading the line");
        let input_lenght = lenght_inserted.trim().parse::<u32>().expect("Error, number not introduced");
        generate_password(input_lenght);
    }
}


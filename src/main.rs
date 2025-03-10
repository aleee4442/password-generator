// Librerias Utilizadas
use rand::Rng; 
use std::io;

fn generar_contra(){
    let mut rng = rand::rng();

    // vector con todos los
    let digitos: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect();

    // Generar la contraseña
    let mut contra = String::new();
    for _ in 0..16 {
        let indice = rng.random_range(0..digitos.len());
        contra.push(digitos[indice]);
    }
    println!("{}",contra);
}


fn main() {
    let mut opcion = String::new();
    println!("Bienvenido al generado de contraseñas seguras");
    println!("Introduce el numero de contrasenias que quieres generar");
    io::stdin().read_line(&mut opcion).expect("Error reading the line");
    let opcion_u32 = opcion.trim().parse::<u32>().expect("Error, numero no introducido");
    for _ in 0..opcion_u32{
        generar_contra();
    }
}


// prueba



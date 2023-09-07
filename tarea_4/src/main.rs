use std::io;

fn main() {
    let mut input = String::new();

    println!("Introduce un numero entre 1 y 1000:");
    io::stdin().read_line(&mut input).expect("Error al leer el numero");

    let num: u32 = input.trim().parse().expect("Por favor introduce un numero");

    if num < 1 || num > 1000 {
        println!("Por favor introduce un numero entre 1 y 1000");
        return;
    }

    let count = count_ones_in_binary(num);
    println!("La representacion binaria de {} es: {:b} y contiene {} veces el numero 1.", num, num, count);
}

fn count_ones_in_binary(mut n: u32) -> u32 {
    let mut count = 0;
    while n > 0 {
        count += n % 2;  // Añade 1 si el residuo es 1, añade 0 si es 0.
        n /= 2;
    }
    count
}

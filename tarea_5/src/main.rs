use std::io;

fn main() {
    println!("Por favor introduce un número menor a 30000 y mayor a 2:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error al leer el número");
    let n: u32 = n.trim().parse().expect("Por favor introduce un número válido");

    println!("Los números no pertenecientes a la secuencia de Fibonacci entre 2 y {} son:", n);
    for i in 2..n {
        if !is_fibonacci(i) {
            print!("{} ", i);
        }
    }
    println!();
}

fn is_perfect_square(x: u32) -> bool {
    let s = (x as f64).sqrt() as u32;
    s * s == x
}

fn is_fibonacci(num: u32) -> bool {
    is_perfect_square(5 * num * num + 4) || is_perfect_square(5 * num * num - 4)
}
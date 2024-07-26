use std::mem::size_of;
use std::time::Instant;

const TAM_FILAS: usize = 7;
const TAM_COLUMNAS: usize = 2;
fn main() {
    std::thread::Builder::new()
        .stack_size(size_of::<f64>() * 2_000_000_000) // El stack es lento para hacer operaciones y sospecho que se puede recalcular mediante programación dinamica
        .spawn(|| {
            println!("\n Por definición:");
            for m in 0..TAM_FILAS-2 {
                print!("\nm={m}   ");
                for n in 0..TAM_COLUMNAS {
                    let inicio = Instant::now();
                    let res = ackermann_recursivo_def(m as u8, n as u128);
                    let dur = inicio.elapsed();
                    print!("Resultado: {}, {:?}", res, dur);
                    if n < TAM_COLUMNAS-1 {
                        print!("  ||  ")
                    }
                }
                println!();
            }
            println!("\n Por programación dinámica:");// Considerar hacer cada uno de los apartados hilos distintos para que se hagan en paralelo
            let mut matriz: [[u128; TAM_COLUMNAS]; TAM_FILAS] = [[0; TAM_COLUMNAS]; TAM_FILAS];
            for m in 0..TAM_FILAS {
                print!("\nm={m}   ");
                for n in 0..TAM_COLUMNAS {
                    let inicio = Instant::now();
                    let res = ackermann_dinamico(m as u8, n as u128, &mut matriz);
                    let dur = inicio.elapsed();
                    print!("Resultado: {}, {:?}", res, dur);
                    if n < TAM_COLUMNAS-1 {
                        print!("  ||  ")
                    }
                }
                println!();
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

fn ackermann_recursivo_def(m: u8, n: u128) -> u128 {
    /* La operación del match es más lenta que la versión con if statements
    let resultado:u128 =match (m,n) {
        (0,_) => n+1,
        (_,0) => ackermann_recursivo_def(m-1,1),
        _ => ackermann_recursivo_def(m-1,ackermann_recursivo_def(m,n-1))
    };
    return resultado;
    */
    return if m == 0 {
        return n + 1;
    } else if m > 0 && n == 0 {
        ackermann_recursivo_def(m - 1, 1)
    } else {
        ackermann_recursivo_def(m - 1, ackermann_recursivo_def(m, n - 1))
    }
}
fn ackermann_dinamico(m: u8, n: u128, matriz: &mut [[u128; TAM_COLUMNAS]; TAM_FILAS]) -> u128 {
    if m as usize >= matriz.len() || n as usize >= matriz[0].len() {
        print!("Error: ");
        return 0; // Devuelve un valor predeterminado si los índices están fuera de los límites
    }
    if matriz[m as usize][n as usize] != 0 { //Index out of bounds, probablemente debido a n, el indice 2 esta fuera de limites para la longitud 2
        return matriz[m as usize][n as usize];
    }
    let resultado:u128;
    if m == 0 {
        resultado=n + 1;
    } else if m > 0 && n == 0 {
        resultado=ackermann_recursivo_def(m - 1, 1);
    } else {
        resultado=ackermann_recursivo_def(m - 1, ackermann_recursivo_def(m, n - 1));
    }
    matriz[m as usize][n as usize]=resultado;
    return resultado
}

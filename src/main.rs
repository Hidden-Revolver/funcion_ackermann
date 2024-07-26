use std::mem::size_of;
use std::time::Instant;
use simple_matrix::Matrix;
const tam_columnas: usize = 2;
const tam_filas: usize = 7;
fn main() {
    std::thread::Builder::new()
        .stack_size(size_of::<f64>() * 2_000_000_000) // El stack es lento para hacer operaciones y sospecho que se puede recalcular mediante programación dinamica
        .spawn(|| {
            for m in 0..tam_filas {
                print!("\nm={m}   ");
                for n in 0..tam_columnas {
                    let inicio = Instant::now();
                    let res = ackermann_recursivo_def(m as u8, n as u128);
                    let dur = inicio.elapsed();
                    print!("Resultado: {}, {:?}", res, dur);
                    if n < tam_columnas {
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
    if m == 0 {
        return n + 1;
    }
    if m > 0 && n == 0 {
        return ackermann_recursivo_def(m - 1, 1);
    }
    if m > 0 && n > 0 {
        return ackermann_recursivo_def(m - 1, ackermann_recursivo_def(m, n - 1));
    }
    return 0;
}
fn ackermann_dinamico(m: u8, n: u128) -> u128 {
    let matriz:Matrix<u128>=Matrix::new(tam_filas,tam_columnas);
    for i in 1..tam_filas+1{
        for j in 1..tam_columnas+1{
            if matriz.get(i,j).unwrap() != &0 {//Continuar por aqui
                unimplemented!();
            }
        }
    }
    n //TODO Metodo por realizar
}

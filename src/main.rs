//use std::io::{stdin};
use std::mem::size_of;
use std::time::{Instant};

fn main() {
/*
    let mut input=String::new();
    let a:u128;
    match stdin().read_line(&mut input) {
        Ok(n) => {
            if n==1 {
                panic!("Tienes que introducir un valor")
            }
            println!("{}", input);//Debug
            if let Some('\n')=input.chars().next_back(){
                input.pop();
            }
            if let Some('\r')=input.chars().next_back() {
                input.pop();
            }
            match input.parse::<u128>() {
                Ok(r) => {
                    a=r;
                }
                Err(e) => {
                    panic!("Error: {}",e);
                }
            }
            println!("Tu opcion seleccionada es: {}",a);
            println!("{}",algoritmo(a));
        }
        Err(e) => {
            panic!("{}",e);
        }
    };
 */
    std::thread::Builder::new()
        .stack_size(size_of::<f64>() * 2_000_000_000)// El stack es lento para hacer operaciones y sospecho que se puede recalcular mediante programación dinamica
        .spawn(||{
            let tam=2;
            for m in 0..7 {
                print!("\nm={m}   ");
                for n in 0..tam{
                    let inicio=Instant::now();
                    let res=ackerman_recursivo_def(m,n);
                    let dur=inicio.elapsed();
                    print!("Resultado: {}, {:?}",res, dur);
                    if n<tam { print!("  ||  ")}
                }
                println!();
            }
        }).unwrap().join().unwrap();
}

fn ackerman_recursivo_def(m:u8,n:u128)->u128{
    if m==0 { return n+1; }
    if m>0 && n==0 {return ackerman_recursivo_def(m-1,1);}
    if m>0 && n>0 {return ackerman_recursivo_def(m-1, ackerman_recursivo_def(m, n - 1));}
    return 0;
}
mod units;
use std::io::{self, Read};
use units::{unidades,unidade_peso,unidade_tempo};
fn main() {

    let peso_da_compra = (5.2, unidades::peso(unidade_peso::quilos));
    let tempo_da_compra = (60, unidades::tempo(unidade_tempo::minuto));


    println!("{:?}",peso_da_compra);
    println!("{:?} minutos",tempo_da_compra);

    let mini_conversor = tempo_da_compra.0 * 60;

    println!("{:?} segundos",mini_conversor);

    let mini_conversor = tempo_da_compra.0 / 60;

    println!("{:?} horas",mini_conversor);

    let mut menu:i8 = 1;

    while (menu != 0) {
        println!("-----------MENU-----------");
        println!("1 - Colocar um valor para converter");
        println!("0 - Sair");

        menu = input_int8() ;
    }

    
}


fn input_int8() -> i8{
    let mut num = String::new();

        io::stdin()
        .read_line(&mut num)
        .expect("Erro");

    num.trim()
    .parse()
    .expect("Erro")
}
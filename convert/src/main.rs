use std::io::{self};
fn main() {

    let mut menu:i8;

    while true {
        println!("-----------MENU-----------");
        println!("0 - Sair");
        println!("1 - Colocar um valor para converter");
        println!("Digite uma opcao:  ");
        menu = input_int8() ;

        if menu==0 {
            break;
        }

        println!("Digite a operacao de conversao completa: ");
        let text = input_string();
        conversor(text);
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

fn input_string() -> String {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Erro");

    num.trim().to_string()
}

fn conversor(texto: String){
    //Quebra a entrada em um vetor de "palavras"
     let args: Vec<&str> = texto.trim()
                                .split_whitespace()
                                .collect();

    if args.len() != 4 {
            println!("Erro: Formato inválido. Verifique o exemplo e tente novamente.");
            return;
        }

     // 4. Extração das partes para variáveis com nomes claros
        let comando = args[0];
        let valor_str = args[1];
        let unidade_origem = args[2];
        let unidade_destino = args[3];

        let valor: f64 = match valor_str.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Erro: '{}' não é um número válido.", valor_str);
               return;
            }
        }; 

        match comando {
            "temperatura" => {
                // Nível 2 da árvore: qual conversão de temperatura?
                match (unidade_origem, unidade_destino) {
                    ("celcius", "fahrenheit") => {
                        let resultado = (valor * 9.0/5.0) + 32.0;
                        println!("Resultado: {}°celcius é igual a {:.2}°fahrenheit\n\n", valor, resultado);
                    },
                    ("fahrenheit", "celcius") => {
                        let resultado = (valor - 32.0) * 5.0/9.0;
                        println!("Resultado: {}°fahrenheit é igual a {:.2}°celcius\n\n", valor, resultado);
                    },
                    ("kelvin", "celcius") => {
                        let resultado = valor - 273.0;
                        println!("Resultado: {}°kelvin é igual a {:.2}°celcius\n\n", valor, resultado);
                    },
                     ("celcius", "kelvin") => {
                        let resultado = valor + 273.0;
                        println!("Resultado: {}°celcius é igual a {:.2}°kelvin\n\n", valor, resultado);
                    },
                     ("fahrenheit", "kelvin") => {
                        let resultado = (valor-32.0)*(5.0/9.0)+273.0;
                        println!("Resultado: {}fahrenheit é igual a {:.2}°kelvin\n\n", valor, resultado);
                    },
                    ("kelvin", "fahrenheit") => {
                        let resultado = (valor-273.0)*1.8 + 32.0;
                        println!("Resultado: {}fahrenheit é igual a {:.2}°kelvin\n\n", valor, resultado);
                    },
                    _ => {
                        println!("Conversão de temperatura inválida. Use 'c' para 'f' ou 'f' para 'c'.\n\n");
                    }
                }
            },
            "distancia" => {
                match (unidade_origem, unidade_destino) {
                    ("km", "milha") => {
                        let resultado = valor / 1.6;
                        println!("Resultado: {} kms é igual a {:.2} milhas\n\n", valor, resultado);
                    },
                    ("km", "metro") => {
                        let resultado = valor * 1000.0;
                        println!("Resultado: {} kms é igual a {:.2} metros\n\n", valor, resultado);
                    },
                    ("milha", "metro") => {
                        let resultado = valor * 1609.0;
                        println!("Resultado: {} milhas é igual a {:.2} metros\n\n", valor, resultado);
                    },
                    ("milha", "km") => {
                        let resultado = valor * 1.6;
                        println!("Resultado: {} milhas é igual a {:.2} kms\n\n", valor, resultado);
                    },
                    ("metro", "milha") => {
                        let resultado = valor / 1609.0;
                        println!("Resultado: {} metros é igual a {:.2} milhas\n\n", valor, resultado);
                    },
                    ("metro", "km") => {
                        let resultado = valor / 1000.0;
                        println!("Resultado: {} metros é igual a {:.2} kms\n\n", valor, resultado);
                    },
                    _ => {
                         println!("Conversão de distância inválida. Use 'km' para 'mi' ou 'mi' para 'km'.");
                    }
                }
            },
            "peso" => {
                match (unidade_origem, unidade_destino) {
                    ("grama", "libra") => {
                        let resultado = valor / 453.0;
                        println!("Resultado: {:.2} gramas é igual a {:.2} libras\n\n", valor, resultado);
                    },
                    ("libra", "grama") => {
                        let resultado = valor * 453.0;
                        println!("Resultado: {:.2} libras é igual a {:.2} gramas\n\n", valor, resultado);
                    },
                    ("libra", "quilo") => {
                        let resultado = valor * 0.453;
                        println!("Resultado: {:.2} libras é igual a {:.2} quilos\n\n", valor, resultado);
                    },
                    ("grama", "quilo") => {
                        let resultado = valor / 1000.0;
                        println!("Resultado: {:.2} gramas é igual a {:.2} quilos\n\n", valor, resultado);
                    },
                    ("quilo", "grama") => {
                        let resultado = valor * 1000.0;
                        println!("Resultado: {:.2} quilos é igual a {:.2} gramas\n\n", valor, resultado);
                    },
                     ("quilo", "libra") => {
                        let resultado = valor / 0.453;
                        println!("Resultado: {:.2} quilos é igual a {:.2} libras\n\n", valor, resultado);
                    },
                    _ => {
                         println!("Conversão de peso inválida. Use 'g' para 'lb' ou 'lb' para 'g'.");
                    }
                }
            },
            "tempo" => {
                match (unidade_origem, unidade_destino) {
                    ("minuto", "hora") => {
                        let resultado = valor / 60.0;
                        println!("Resultado: {} minutos é igual a {:.4} horas", valor, resultado);
                    },
                    ("minuto", "segundo") => {
                        let resultado = valor * 60.0;
                        println!("Resultado: {} minutos é igual a {:.2} segundos", valor, resultado);
                    },
                     ("segundo", "minuto") => {
                        let resultado = valor / 60.0;
                        println!("Resultado: {} segundos é igual a {:.2} minutos", valor, resultado);
                    },
                     ("segundo", "hora") => {
                        let resultado = valor / 3600.0;
                        println!("Resultado: {} segundos é igual a {} segundos", valor, resultado);
                    },
                     ("hora", "minuto") => {
                        let resultado = valor * 60.0;
                        println!("Resultado: {} horas é igual a {:.2} minutos", valor, resultado);
                    },
                     ("hora", "segundo") => {
                        let resultado = valor * 3600.0;
                        println!("Resultado: {} horas é igual a {} segundos", valor, resultado);
                    },
                    _ => {
                         println!("Conversão de peso inválida. Use 'g' para 'lb' ou 'lb' para 'g'.");
                    }
                }
            },
            // Se o comando não for nenhum dos anteriores
            _ => {
                println!("Erro: Comando '{}' desconhecido. Use 'temp', 'dist' ou 'weight'.", comando);
            }
        }
    }

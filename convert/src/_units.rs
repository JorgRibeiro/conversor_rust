#[derive(Debug)]
pub enum unidades{
    temperatura(unidade_temperatura),
    tempo(unidade_tempo),
    peso(unidade_peso),
    distancia(unidade_distancia),
}

#[derive(Debug)]
pub enum unidade_temperatura {
    celcius,
    fahrenheit,
    kelvin,
}

#[derive(Debug)]
pub enum unidade_tempo {
    segundo,
    minuto,
    hora,
}
#[derive(Debug)]
pub enum unidade_distancia {
    km,
    milha,
    metro,
}
#[derive(Debug)]
pub enum unidade_peso {
    gramas,
    quilos,
    libras,
}
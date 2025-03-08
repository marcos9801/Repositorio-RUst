fn main(){
    let x = 5;
    //ambiente
    {
        let x  = x+1;
        println!("El valor x: {}", x);
    }
    println!("El valor x: {}", x);

    //variables inmutables
    let mut y = 5;
    y = 20;

    //imprimir mensaje
    println!("Hello, world!");
    let entero: i32 = 42;
    let flotante: f64 = 3.1416;
    let booleano: bool = true;
    let ccaracter: char = 'a';
    //creacion de tupla
    let firulais: (i32, f64, char) = (42, 6.283185307179586, 'c');
    let arreglo: [i32; 3] = [1, 2, 3];
    println!("Tupla(firulais) forma 1: {:?}", firulais);
    println!("Tupla(firulais) forma 2: ({}, {}, {})", firulais.0, firulais.1, firulais.2);
}

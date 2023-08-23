fn main() {
    // let var = '2';
    // let mut var2 = 2;
    // //hace refencia a var2
    // let cambio_valor = &mut var2;
    // //le incrementamos el valor de var2
    // *cambio_valor += 5;
    // let var3 : u32 = 123;
    // let var4: char = 'H';
    // let PI: f32 = 3.45;
    // let verdadero: bool = true;


    // // Tuplas
    // let tupla = (1,2,3,'a');
    // let tupla2: (i32,char, char, i32) = (1,'a','b',5);
    // println!("Esto es una tupla {}", tupla.1);

    // // Arrays
    // let array = [1,2,3,4];
    // let array2: [i32; 4] = [1,2,3,4];
    // println!("Esto es un array {}", array[1]);
    // for num in array2 {
    //     println!("El numero es {}", num);
    // }

    // //Vector
    // let vector = vec![1,2,3,4,5];
    // for vec in &vector{
    //     println!("Vector: {}", vec);
    // }
    // func(7);
    // let valor_retornar = retorna(12);
    // println!("El valor que se retorno {}", valor_retornar);

    // condiciones(3);
    //propiedades();
    // let prop = String::from("Algun string");
    // toma_propiedad(prop.clone());
    // println!("{}", prop);

    // let mut s = String::from("Algun string");
    // modificacion(&mut s);

    // println!("{}", s);

    // let mut cadena = String::from("Algun string y esto es run");
    // //let palabra = primeras_palabras(&cadena)
    // let algun = &cadena[0..5];
    // let strng = &cadena[6..12];

    // println!("la primera palabra es {}", primeras_palabras(&cadena));

    let cadena = String::from("Esto es una cadena");
    let numero:i32 = 32;

    let cadena2 = &cadena;
    let numero2 = numero;
    println!("esto es cadena 2 {}", cadena2);
    println!("esto es cadena 1 {}", cadena);
    println!("este es numero 2 {}", numero2);
    println!("este es numero 1 {}", numero2);

}

// fn toma_propiedad(cadena: String){
//     println!("{}", cadena);
// }

// fn propiedades(){
//     let cadena = String::from("Hola amigo");
//     let cadena1 = &cadena;

//     println!("{}", cadena1);
//     println!("{}", cadena);
// }


// fn func(i: u32){
//     let b = 32;

//     let k = {
//         let c = b + 1;

//         c + 1
//     };


//     println!("Esto es una funciona {}", k);
// }

// fn retorna(i:i8) -> i8 {
//     13 + i
// }


// fn condiciones(a:i32){
//     if a > 5{
//         println!("Es mayor");
//     }else{
//         println!("Es menor");
//     }

//     let array = [1,2,3,4,5,6];
//     for ar in array{
//         println!("Los valores: {}", ar);
//     }


// }

//Referencia y prestamos

// fn modificacion(cadena: &mut String) {
//     cadena.push_str("Tengo agregado");
    
// }

//Segmentos

// fn primeras_palabras(cadena: &str) -> &str {
//     let byte = cadena.as_bytes();
//     for(i, &item) in bytes.iter().enumerate(){
//         if item == b' ' {
//             return &cadena[0..i];
//         }
//     }
//     &cadena[..]
// }


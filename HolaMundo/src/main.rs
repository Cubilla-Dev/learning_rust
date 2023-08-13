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
    func(7);
    let valor_retornar = retorna(12);
    println!("El valor que se retorno {}", valor_retornar)
}

fn func(i: u32){
    let b = 32;

    let k = {
        let c = b + 1;

        c + 1
    };


    println!("Esto es una funciona {}", k);
}

fn retorna(i:i8) -> i8 {
    13 + i
}
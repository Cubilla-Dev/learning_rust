fn main() {
    let mut name: String = String::new();
    println!("Cual es tu nombre :");
    std::io::stdin().read_line(&mut name).unwrap();
    let x:usize = caracteres_to_name(&name);
    add_to_string(&mut name);
    println!("el nombre es: {} y se agrego :{}", name, x);

}

fn add_to_string(s: &mut String) {
    s.push_str("Dato agregado")
}

fn caracteres_to_name(s: &String) -> usize{
    s.len()
}

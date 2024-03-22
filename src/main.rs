fn main() {
    let name: &str = "toyin";
    // let name: bool = true;

    let mut var2: i32 = 30;
    println!("original => {}", var2);
    var2 = 40;
    println!("modified => {}", var2);

    println!("\n");

    const NEW_VARIABLE: i32 = 32;
    println!("Hello, world! {}", name);

    println!("{}", NEW_VARIABLE)
}

fn main() {
    println!("Hello, world!");

    let string_var = String::from("This string");
    let chars = string_var.chars();
    
    //print(string_var);  // Does not work
    
    //print!(string_var); // error: format argument must be a string literal
    
    print!("{}", string_var);

    //print!("{}", chars); > // error[E0277]: `Chars<'_>` doesn't implement `std::fmt::Display`

    println!("{}", chars.count());

    println!("Bla!"); // OK


}

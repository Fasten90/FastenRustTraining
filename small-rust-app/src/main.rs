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

    /* Is this a multiline comment?
    */

    // https://doc.rust-lang.org/stable/rust-by-example/primitives.html

    let mut var1: bool = true; // If mut missed, the line at below report: error[E0384]: cannot assign twice to immutable variable `var1`
    // Lol bro, I give a warning to here due the: warning: value assigned to `var1` is never read
    //var1 = false;


    //println('is this working?'); // error[E0762]: unterminated character literal
    println!("bool: {}", var1); // it will print "true"
    var1 = false;
    if var1 {
        println!("But I read it!"); // Suppress the warnings: warning: value assigned to `var1` is never read
    }

    //var1 = 5; // error[E0308]: mismatched types
    //var1 = 1; // same

/*
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;
*/

}

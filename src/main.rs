fn main() {

    // comentarios con rust
    /*
    Multiple comentarios
    con rust
    s
    da
    asd
     */
    println!("Hola Rust en español"); // ";" muy importante si no el compilador molestará y no lo va compilar.

    // #[warn(unused_doc_domments)] - error de comentarios
    println!("Hello, world!");

    // Como se escribe una variable ?

    // #[warn(ununsed_variable)] -- Cuando no se usa una variable
    let my_string = "Hello World";

    println!("Esta es mi variable: {}", my_string);
}

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
    let mut my_string: &str = "Hello World";

    println!("{my_string}");
   // my_string = "CAMBIAMOS EL VALOR"; cannot assign twice to immutable variable `my_string`

   // las variables por defecto son inmutables

   my_string = "Aqui cambio la cadena de texto";

    println!("Esta es mi variable: {}", my_string);
    // my_string = 6; tipado fuerte

    // expected String, found &strrust-analyzerE0308 
    // las cadenas de texto se representan de dos formas

    // &str cadena utf8 de una longitud fija reserva el maximo de mem0ria

    // string vamos redifiniendo el tamaño de la cadena de texto
    // let new_variable: String = "Esto es una cadena de texto"; --> rust se va quejar


    let redifiniendo_string = String::from("Valor de la cadena de textp");

    println!("{redifiniendo_string}");

    let mut esta_variable_no_infiere_valor_fijo = String::from("Valor no fijo");

    println!("{esta_variable_no_infiere_valor_fijo}");
    // tendras que convetir nuevamente el string
    esta_variable_no_infiere_valor_fijo = String::from("Modificamos su valor");
    println!("{esta_variable_no_infiere_valor_fijo}");

    let mut aunmento_de_bytes = String::from("Hola");

    println!("{}", aunmento_de_bytes);
    // aunmento de caracteres
    aunmento_de_bytes = String::from("Hola,Mundo");

    println!("{}", aunmento_de_bytes);


    let mut bytes_fijos = "No puede aunmentar la cadena establecida";

    println!("{}", bytes_fijos);

    bytes_fijos = "menos";

println!("{}", bytes_fijos);
}

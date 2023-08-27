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

    let my_variable_inmutable: &str = "Variable inmutable por defecto";
    println!("{}", my_variable_inmutable);

    // integer float f64
    let value_integer: f64 = 10.10;

    print!("{}", value_integer);

    // intenger int i34
    let mut integer_int: i32 = 10;
    println!("{}", integer_int);

    integer_int = integer_int + 2;

    println!("{}", integer_int);

    let my_bool: bool = true;

    println!("{} - {}", my_bool, false);


    // costantes

    // es obligatorio definir el tipo de dato de una constante
    // las constantes no pueden mutar
    const MY_CONST: f64 = 3.14;
    const MY_BOOL_CONST: bool = true;
    // MY_CONST = ""; No funciona error
    println!("{} - {}", MY_CONST, MY_BOOL_CONST);


    // control de flujos

    if MY_BOOL_CONST {
        println!("Funciona");
    }
    if MY_CONST == 3.14 {
        println!("Es igual");
    }

    if true {
        println!("true");
    } else {
        println!("false");
    }

    if true {
        println!("true");
    } else if true {
        println!("true");
    } else {
        println!("false");
    }

    if true && true {
        println!("operadores");
    }

    if false || true {
        println!("operadores ||");
    }
    // interesantes
    // if let values = 10 == 10 {
    //     println!("OMG - {}", values);
    // }

    
}

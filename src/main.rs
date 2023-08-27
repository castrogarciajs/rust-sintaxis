use std::io;

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

    // listas

    let mut my_list: Vec<&str> = vec!["string", "solo es string no acepta numeros"];

    my_list.push("value"); // añadiendo valor
    println!("{:?}", my_list);
    my_list = vec!["nuevo vector"];
    println!("{:?}", my_list);
    println!("{:?}", my_list[0]);

    // cuando usamos constantes en rust en vectores debe saber cuantos elemento tendra la constante se leen en tiempo de compilacion

    // para usar constante con matrices debes usar esta forma especifcando tipo de dato y cantidad de elementos
    const MY_VECTOR_INTEGER: [i64; 2] = [10, 20];

    println!("{:?}", MY_VECTOR_INTEGER);

    let mut my_string_new = String::new();

    my_string_new.push_str("sstrig");
    my_string_new.push_str("sstrigasdasd");
    // pense que se iba quejar el compilador ajajajaja
    let string_vacio = "";
    println!("{}", string_vacio);
    println!("Escribe: ");

    let mut valor: String = String::new();

    io::stdin()
        .read_line(&mut valor)
        .expect("Ha fallado al leer");

    println!("Yo valor es : {}", valor);

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Hubo un error al leer");

    const NUMBER_EQUAL: i64 = 10;
    const NUMBER_EQUAL_COMPARE: i64 = 12;

    if NUMBER_EQUAL < NUMBER_EQUAL_COMPARE {
        println!("{}", true);
    } else {
        println!("{}", false);
    }

    let number_par: Vec<i32> = vec![10, 20,30];

    println!("{:?}", number_par);

    let mut response_client = String::new();

    println!("Escribe una entrada");
    io::stdin().read_line(&mut response_client).expect("Error al escribir");

    println!("{}", response_client);
}

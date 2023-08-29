use std::collections::{HashMap, HashSet};
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

    let len_string = my_string.len();

    println!("{}", len_string);
    // let mut string_bytes = my_string.bytes();

    // println!("{}", string_bytes);
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

    my_list.push("value"); // añadiendo valor push
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

    if NUMBER_EQUAL < NUMBER_EQUAL_COMPARE && NUMBER_EQUAL > NUMBER_EQUAL_COMPARE {
        println!("{}", true);
    } else {
        println!("{}", false);
    }

    let number_par: Vec<i32> = vec![10, 20, 30];

    println!("{:?}", number_par);

    let mut response_client = String::new();

    println!("Escribe una entrada");
    io::stdin()
        .read_line(&mut response_client)
        .expect("Error al escribir");

    println!("{}", response_client);

    let vector_bool: Vec<bool> = vec![true, false, true, false, true, false];

    println!("{:?}", vector_bool);
    println!("{:?}", vector_bool[0]);
    println!("{:?}", vector_bool[1]);
    println!("{:?}", vector_bool[2]);

    // tipos de datos
    // variables mut e inmutables
    // constantes
    // control de flujo
    // vectores

    // 1. primeros ejercicios de rust

    // #1. Declara variables de diferentes tipos de datos: enteros, flotantes, booleanos y cadenas.
    //         Asigna valores a estas variables y observa cómo se comportan.

    // -------------------------------------- SOLUCION --------------------------------- //

    let type_string_variable = "Variable de tipo string";
    let type_number_integer_variable = 10;
    let type_number_float_variable = 3.14;
    let type_bool_variable = true;

    println!(
        "string: {} - number: {} - float: {} - bool: {}",
        type_string_variable,
        type_number_integer_variable,
        type_number_float_variable,
        type_bool_variable
    );

    // #2 Declara una variable mutable y cambia su valor.
    //        Intenta cambiar el valor de una variable inmutable y observa el error que obtienes.

    // -------------------------------------------SOLUCION --------------------------------------//

    let mut variable_mutable_que_va_a_cambiar_su_valor = "Valor ha cambiar";

    println!(
        "primer valor: {}",
        variable_mutable_que_va_a_cambiar_su_valor
    );
    variable_mutable_que_va_a_cambiar_su_valor = "Valor cmbiado";

    println!(
        "nuevo valor: {}",
        variable_mutable_que_va_a_cambiar_su_valor
    );

    let variable_inmutable_por_defecto = "inmutable";

    // variable_inmutable_por_defecto = "error"; // error `#[warn(unused_assignments)]`
    println!("{}", variable_inmutable_por_defecto);

    // #3 Declara una constante y asígnale un valor.
    //    Intenta cambiar el valor de una constante y observa el error que obtienes.

    const VALOR_CONSTANTES_NO_PUEDE_CAMBIAR: &str = "string constante";

    // VALOR_CONSTANTES_NO_PUEDE_CAMBIAR = "error";
    println!("{}", VALOR_CONSTANTES_NO_PUEDE_CAMBIAR);

    // sets

    let mut my_first_set_in_rust: HashSet<&str> = ["sebastian", "garcia"].into_iter().collect();
    let mut sets_with_number: HashSet<i64> = [10, 20, 30].into_iter().collect();
    let mut sets_with_bool: HashSet<bool> = [true, false].into_iter().collect();

    sets_with_number.insert(10);
    my_first_set_in_rust.insert("garcia");
    sets_with_bool.insert(false); sets_with_bool.remove(&true);
    println!("{:?}", my_first_set_in_rust);
    println!("{:?}", sets_with_number);
    println!("{:?}", sets_with_bool);

    // maps
    let maps: HashMap<&str, i32> = vec![("sebastian", 10), ("jhoan", 32)].into_iter().collect();

    println!("{:?}", maps);

    // bucles

let iterator_vecto: Vec<&str> =vec!["A", "B", "C"]; 
let string_iterator: String = String::from("SEBASTIAN");


println!("{}", string_iterator);
for values in iterator_vecto {
    println!("{}", values);
}
for values_set in my_first_set_in_rust {
    println!("{}", values_set);
}

for (key, values) in maps  {
    println!("key: {} - values: {}", key, values);
}
// no iterator String and str
// for string_values in string_iterator {
//     println!("{}", string_values);
// }

// usa el metodo chars para indicarle a rust que son caracteres
for (index, string_values) in string_iterator.chars().enumerate() {
    println!("{} - {}", string_values, index);
}

let mut my_counter: usize = 0;

while my_counter < my_list.len() {
    my_counter += 1;
    println!("{}", my_counter);
}
my_first_function();
}


fn my_first_function() {
    println!("First funtion in rust");
}
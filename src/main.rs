use std::collections::{HashMap, HashSet};
use std::io;
use crate::garden::vegetables::Asparagus;
use crate::module::hello_world;


//     │   ├── module.rs (Módulo de Paquete)
mod module;
mod system;

// La línea mod garden; le dice al compilador que incluya el código que encuentra en src/garden.rs, que es:
pub mod garden;

fn main() {
    // comentarios con rust
    /*
    Multiple comentarios
    con rust
    s
    da
    asd
     */
    system::system_add();
    let plant = Asparagus {};
    println!("{:#?}", plant);
    println!("Probando NVIM");
    // desacer en nvim
    println!("Hola Rust en español"); // ";" muy importante si no el compilador molestará y no lo va compilar.
    println!("hola nvim");
    // practicando nvim
    // flujio
    // de
    // neovim
    //
    // #[warn(unused_doc_domments)] - error de comentarios
    println!("Hello, world!");

    hello_world();
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
    sets_with_bool.insert(false);
    sets_with_bool.remove(&true);
    println!("{:?}", my_first_set_in_rust);
    println!("{:?}", sets_with_number);
    println!("{:?}", sets_with_bool);

    // maps
    let maps: HashMap<&str, i32> = vec![("sebastian", 10), ("jhoan", 32)].into_iter().collect();

    println!("{:?}", maps);

    // bucles

    let iterator_vecto: Vec<&str> = vec!["A", "B", "C"];
    let string_iterator: String = String::from("SEBASTIAN");

    println!("{}", string_iterator);
    for values in iterator_vecto {
        println!("{}", values);
    }
    for values_set in my_first_set_in_rust {
        println!("{}", values_set);
    }

    for (key, values) in maps {
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

    // funciones
    my_first_function();

    //

    fn three_function() {
        println!("three function");
    }
    three_function();

    // tipo caracter
    // El tipo char de Rust es el tipo alfabético más primitivo del lenguaje. Aquí hay algunos ejemplos de declarar valores char:

    let c = 'X';
    let z = 'Z';

    println!("{} - {}", c, z);
    // tipo tuplas

    let tup: (i32, f64, u8) = (500, 3.14, 2);
    let (x, y, z) = tup;

    let tup_index_value = tup.0;
    let tup_index_two_value = tup.1;

    println!("{:?}", tup);
    println!("{} - {} - {}", x, y, z);

    println!("{} - {}", tup_index_value, tup_index_two_value);

    // tipo array

    let number_array = [1, 2, 3, 4, 5, 6];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let array_init = [3; 5];

    println!("{:?}", number_array);
    println!("{:?}", months);
    println!("{:?}", array_init);

    let a = [1, 2, 3, 4, 5];
    let mut client_variable = String::new();

    io::stdin().read_line(&mut client_variable).expect("Error");

    let client_variable: usize = client_variable.trim().parse().expect("no es un numero");

    let elem = a[client_variable];

    println!("{}", elem);
    let x = five();

    println!("The value of x is: {x}");

    // loops infinite

    // esto es un bucle infinito
    loop {
        println!("again");
        break;
    }
    let mut counter_loop = 0;

    let result = loop {
        counter_loop += 1;
        if counter_loop == 10 {
            break counter_loop * 2;
        }
    };

    println!("{}", result);

    let mut coun_break_loop = 0;
    'breaking_loop: loop {
        println!("count = {}", coun_break_loop);

        let mut remaning = 10;
        loop {
            println!("remaning = {}", remaning);

            if remaning == 9 {
                break;
            }
            if coun_break_loop == 2 {
                break 'breaking_loop;
            }
            remaning -= 1;
        }

        coun_break_loop += 1
        // probando neovim
    }
    println!("Neovim");

    // Ownership El ownership es un conjunto de reglas que gobiernan cómo un programa de Rust administra la memoria.
    /*Primero, echemos un vistazo a las reglas de ownership. Mantenga estas reglas en mente mientras trabajamos a través de los ejemplos que las ilustran:

    Cada valor en Rust tiene un propietario.
    Solo puede haber un propietario a la vez.
    Cuando el propietario sale del alcance, el valor se descartará.



     * */

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(usState::Alabama));
    // let mut n = "s";

    let s1 = String::from("Clone string");

    let s2 = s1.clone();

    println!("{}", s2);

    let owner = String::from("strnig");

    tomar_ownership(owner);

    let xs = 5;
    hacer_una_copia(xs);

    //  Este signo ampersands (&) representa referencia, y te permiten referirte a algún valor sin tomar la propiedad de él. La Figura 4-5 representa este concepto.
    let usize_len = calcular_longitud(&s1);

    let mut s = String::from("hola");

    println!(" La longitud de {} es: {}", s1, usize_len);
    modificar(&mut s);

    // Este error dice que este código es inválido porque no podemos prestar s como mutable más de una vez a la vez. El primer préstamo mutable está en r1 y debe durar hasta que se use en el println!, pero entre la creación de esa referencia mutable y su uso, intentamos crear otra referencia mutable en r2 que presta los mismos datos que r1.
    // let mut s = String::from("hola");
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // El tipo slice

    // Los Slices te permiten referenciar a una secuencia contigua de elementos en una colección en lugar de la colección completa. Un slice es una especie de referencia, por lo que no tiene ownership.

    let mut s = String::from("Hola Mundo");

    let word = first_word(&s);
    println!("{}", word);
    s.clear();

    let slice_string: String = String::from("Hello, World");
    let len = slice_string.len();

    let hello = &slice_string[0..5]; // referencia a un String conocido como slice();
    let slice = &slice_string[2..len];

    println!("{} - slice: {}", hello, slice);

    first_word_slice(&s);

    let array_slice = [1, 2, 3, 4, 5, 6];

    let slice_array = &array_slice[1..3];

    println!("{:?}", slice_array);

    assert_eq!(slice_array, &[2, 3]);
    let string_collect = "hola mundo";

    let chars = string_collect.chars().rev();

    let string_iter: String = chars.collect();

    println!("{}", string_iter);

    let some_username = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@example.com"),
        sing_in_count: 1,
    };

    let other_user = User {
        email: String::from("otherusername@example.com"),
        ..some_username
    };
    println!("{} - {}", some_username.email, other_user.email);
    build_user(String::from("username-build"), some_username.email);

    enum_example();
    route(IpAddrKind::V4);

    value_in_cents(Coin::Quarter(usState::Alaska));

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);

    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

fn first_word(s: &String) -> usize {
    let bytes_string = s.as_bytes();

    for (i, &iter_string) in bytes_string.iter().enumerate() {
        if iter_string == b' ' {
            return i;
        }
    }

    s.len()
}
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn five() -> i32 {
    5
}
fn my_first_function() {
    println!("First funtion in rust");
    second_function();
}

fn second_function() {
    println!("Second funtion");
}

// struct MyStruct {
//     name: &str,
//     age: i32,
// }

// impl MyStruct {
//     fn newConstructor(name: &str, age: i32) {
//         MyStruct { name, age }
//     }
// }
fn modificar(un_string: &mut String) {
    un_string.push_str(", Mundo")
}

fn tomar_ownership(un_string: String) {
    // un_string aparece en el ámbito
    println!("{}", un_string);
} // Aquí termina el ámbito, un_string es destruido con drop.
  // La memoria es liberada.

fn hacer_una_copia(un_entero: i32) {
    // un_entero aparece en el ámbito
    println!("{}", un_entero);
}
fn calcular_longitud(s: &String) -> usize {
    s.len()
}

struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

impl User {
    fn user_completed(&self) -> bool {
        self.active
    }
    fn is_auth(&self) -> u64 {
        self.sing_in_count
    }
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

/*

odas las funciones definidas dentro de un bloque impl se llaman funciones asociadas porque están asociadas con el tipo nombrado después del impl. Podemos definir funciones asociadas que no tengan self como su primer parámetro (y, por lo tanto, no sean métodos) porque no necesitan una instancia del tipo con el que trabajar. Ya hemos usado una función como esta: la función String::from que está definida en el tipo String.*/
impl Rectangle {
    // métodos de instancia
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }

    // funciones asocidas
    fn square(size: u32) -> Self {
        Self {
            width: size,
            heigth: size,
        }
    }

    fn fn_greet(string: String) -> String {
        println!("Hola {}", string);
        string
    }
}
fn build_user(username: String, email: String) {
    let rect_one = Rectangle {
        width: 10,
        heigth: 15,
    };
    let rect_two = Rectangle {
        width: 10,
        heigth: 15,
    };
    rect_one.can_hold(&rect_two);
    println!("{} {}", rect_one.area(), rect_one.width());

    let user_build = User {
        active: true,
        username,
        email,
        sing_in_count: 1,
    };
    user_build.user_completed();
    user_build.is_auth();
    Rectangle::square(10);
    Rectangle::fn_greet(String::from("Sebastian"));
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{} {:?} {:?}", user_build.username, black, origin);

    let subjet = AlwaysEqual;
    println!("{:?}", subjet);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
enum IpaddrKindVariant {
    V4(String),
    V6(String),
}
#[derive(Debug)]
struct IpAdrr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
struct IpAddrVariant {
    kind: IpaddrKindVariant,
}
#[derive(Debug)]
enum VariantType {
    VariantIp(u32, u32, u32),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[derive(Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

// Declare enum for Car transmission type
#[derive(Debug)]
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    _Manual,
    _SemiAuto,
    _Automatic,
}
impl Message {
    fn call(&self) {
        // method nobody
    }
}
struct _QuitMessage; // unit struct
struct _MoveMessage {
    x: i32,
    y: i32,
}
struct _WriteMessage(String); // tuple struct
struct _ChangeColorMessage(i32, i32, i32); // tuple struct
fn enum_example() {
    let version_six = IpAddrKind::V6;
    let version_four = IpAddrKind::V4;

    let varian_six = IpaddrKindVariant::V6(String::from("::01"));
    VariantType::VariantIp(10, 0, 0);

    let home = IpAdrr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAdrr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let varian_addr = IpAddrVariant {
        kind: IpaddrKindVariant::V4(String::from("variant")),
    };
    println!(
        "{:?} {} {:?} {:?}",
        home.kind, home.address, varian_six, varian_addr.kind
    );
    println!(
        "{:?} {:?} {:#?} {:#?} {:#?}",
        version_six, version_four, home, loopback, varian_addr
    );
}
fn route(ip_kind: IpAddrKind) {
    let m = Message::Write(String::from("hello"));
    m.call();
    println!("{:?}", ip_kind);
}
fn _car_factory(color: String, transmission: Transmission, convertible: bool) {
    // Use the values of the input arguments
    // All new cars always have zero mileage
    // let car: Car = todo!("Create an instance of a `Car` struct");
    let car_new = Car {
        color,
        convertible,
        mileage: 10,
        transmission,
    };

    println!("{:#?}", car_new);
}

enum _Option<T> {
    None,
    Some(T),
}

#[derive(Debug)]
enum usState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(usState),
}

fn value_in_cents(coin: Coin) -> u8 {
    plus_one(Some(5));
    plus_one(None);
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:#?}", state);
            25
        }
    }
}

fn plus_one(i: Option<i32>) -> Option<i32> {
    match i {
        None => None,
        Some(i) => Some(i * 1),
    }
}

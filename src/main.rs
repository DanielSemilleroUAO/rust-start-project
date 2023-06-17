use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use rand::random;

fn main() {
    let number: u8 = random();
    println!("Tu numero es {}", number);

    // variables -> las variables son inmutables
    let x = 8;
    // variable mutable
    let mut y = 10;
    println!("El valor de nuestra variable es: {}", y);
    println!("El valor de nuestra variable es: {}", x);
    // Si se quiere cambiar el valor de una variable inmutable debe ser:
    let x = 5;
    y = 5;
    println!("El valor de nuestra variable es: {}", x);
    println!("El valor de nuestra variable es: {}", y);
    // constantes
    const NUMERO_ESPACIOS: i32 = 3;
    println!("Esta es una variable constante: {}", NUMERO_ESPACIOS);
    // Rust - tipo estático
    // tipado dinámico -> ejemplo javascript - python
    // Integer
    /*
     Se basa en:
     signed -> valores negativos y positivos
     unsigned -> solo valores positivos
     8 bit -> i8 - u8
     16 bit -> i16 - u16
     ...
     128 bit -> i128 - u128
     arch -> isize - usize
    */
    let entero: i8 = 23;
    let entero2: u8 = 40;
    let entoro3: i8 = -48;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;

    // Floating point
    let float1 = 5.0; // f64
    let float2: f32 = 12.432; //f32

    // Boolean
    let verdadero = true;
    let falso: bool = false;

    // Caracter
    let caracter = 'a';
    let emoji = '🥳';

    // Compound types - arrays, tupla
    // tupla
    let tupla = ('h', 23, -45, 0.222);
    let tupla2: (char, i64, i32, f64) = ('a', 23, -45, 0.222);
    let (x, y, z, w) = tupla2;
    println!("El valor de x: {}, y:{}, z: {}, w: {}", x, y, z, w);

    // arreglos
    let arreglo = [1, 2, 3, 4, 5];
    println!("El valor: {}", arreglo[2]);
    let arreglo2: [i128; 5] = [1, 2, 3, 4, 5];

    // strings
    let nombre = "Daniel Delgado";

    let apellido: String = "Delgado".to_string();
    //let direccion: String::new();
    //direccion = "mi casa".to_string();

    // funciones
    mostrar_bienvenida();
    let numero = seleccion_numero(8);
    println!("Tu numero es {}", numero);

    let numero2 = { 10 };
    println!("Tu numero es {}", numero2);

    saludar_con_nombre("Daniel");

    let user = Usuario {
        nombre: "Daniel".to_string(),
        email: "daniel@gmail.com".to_string(),
        edad: 25,
        activo: true,
        pais: "Colombia".to_string(),
    };

    println!("Usuario nombre {}", user.nombre);

    let user2 = nuevo_usuario("Daniel", "daniel@gmail.com");
    let mut user3 = Usuario {
        nombre: "Juan".to_string(),
        email: "juan@gmail.com".to_string(),
        ..user2
    };

    user3.edad = 12;
    println!("Usuario edad {}", user3.calcular_edad());

    // tuple structs
    struct Point(i32, i32, i32);
    let pointA = Point(12, 21, 2);

    // enums
    let role = UserRoles::ADMIN;

    let website = Website::URL("https://example.com".to_string());

    let access = hasAccess(role);

    // Tipos generic
    let opcion = Option::Some(32);

    // traits
    let gato1 = Gato;
    println!("{}", gato1.di_hola());
    let persona = Humano;
    println!("{}", persona.di_hola());

    // iterators
    let s = [1, 2, 3, 4];
    for x in s.iter() {
        println!("{}", x + 1);
    }

    // closures -> lambda. Función en linea
    // | = pipe
    let sumapipe = |nro: i32| nro + 2;
    println!("{}", sumapipe(23));
    let sum = sumanr_uno;
    println!("{}", sum(4));

    // Prelude
    let mi_struc: MiEstructura;
    let algo: HashSet<i32>;
    let mapa: HashMap<i32, i32>;

    let opcion: Option<i32>;

    // Flujos de control
    // if
    let number = 5;
    if number == 5 {
        println!("Es cinco")
    } else if number == 3 {
        println!("Es tres")
    } else {
        println!("No es cinco")
    }

    let resultado = if number > 5 { 1000 } else { 0 };
    println!("{}", resultado);
    //loop

    let mut counter = 0;
    let result = loop {
        println!("loop");
        if counter == 10 {
            break counter;
        }
        counter += 1;
    };
    println!("result: {}", result);

    while counter > 0 {
        println!("loop");
        counter -= 1;
    }

    // for
    let arreglo = [0, 1, 2, 3, 4];
    for element in arreglo.iter() {
        println!("Elemento: {}", element);
    }

    for elemento in 0..arreglo.len() {
        println!("Elemento: {}", elemento);
    }

    for elemento in (0..arreglo.len()).rev() {
        println!("Elemento: {}", elemento);
    }

    // if-let
    let edad: Option<i32> = Some(30);
    match edad {
        Some(edad) => println!("edad {}", edad),
        _ => (),
    }

    if let Some(value) = edad {
        println!("Edad {}", value)
    }

    // while let
    let mut mensajes = Some(10);

    while let Some(value) = mensajes {
        if value > 0 {
            println!("Tienes mensajes no leidos");
            mensajes = Some(value - 1);
        } else {
            println!("No hay mensajes nuevos");
            mensajes = None;
        }
    }

    // let -else
    let algun_numero = Some(100);
    let Some(numero) = algun_numero else {
        panic!("EL número no es valido")
    };

    // labeled blocks
    'primer_bloque: {
        let algo = 3;
        println!("Hola {}", algo);
        if algo > 30 {
            break 'primer_bloque;
        }
    }
}

struct MiEstructura {}

fn sumanr_uno(nro: i32) -> i32 {
    nro + 1
}

fn nuevo_usuario(nombre: &str, email: &str) -> Usuario {
    return Usuario {
        nombre: nombre.to_string(),
        email: email.to_string(),
        edad: 0,
        activo: false,
        pais: "".to_string(),
    };
}

// funciones

fn saludar_con_nombre(arg: &str) {
    println!("Hola {}", arg);
}

fn seleccion_numero(numero: i32) -> i32 {
    numero
}

fn mostrar_bienvenida() {
    println!("Bienvenidos a Rust");
}

// Structs
#[derive(Default, Debug)]
struct Usuario {
    nombre: String,
    email: String,
    edad: i32,
    activo: bool,
    pais: String,
}

impl Usuario {
    fn calcular_edad(&self) -> i32 {
        let actual = 2021;
        return self.edad * actual;
    }
}

enum UserRoles {
    BASIC,
    ADMIN,
}

enum Website {
    URL(String),
    INSTAGRAM(String),
}

fn hasAccess(user_role: UserRoles) -> bool {
    match user_role {
        UserRoles::ADMIN => true,
        UserRoles::BASIC => false,
    }
}

struct Point<T, V> {
    x: T,
    y: V,
}

fn soyGeneric<T, V>(punto: Point<T, V>) {}

// Option
/*
enum Option<T> {
    Some(T),
    None
}
 */

// Traits
struct Humano;
struct Gato;

trait Hablar {
    fn di_hola(&self) -> String;
    fn idioma() -> String {
        "No tengo idioma".to_string()
    }
}

impl Hablar for Humano {
    fn di_hola(&self) -> String {
        "Hola soy humano".to_string()
    }
    fn idioma() -> String {
        "Hablo humano".to_string()
    }
}

impl Hablar for Gato {
    fn di_hola(&self) -> String {
        "Hola soy un gato".to_string()
    }
}
